// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use std::borrow::Borrow;
use std::sync::Arc;

use cid::Cid;
use fvm_ipld_blockstore::Blockstore;
use fvm_ipld_hamt::Error;
use fvm_shared::clock::ChainEpoch;
use fvm_shared::deal::DealID;
use fvm_shared::runtime::traits::HashAlgorithm;
use fvm_shared::HAMT_BIT_WIDTH;

use super::Set;
use crate::{make_empty_map, make_map_with_root, parse_uint_key, u64_key, Map};

/// SetMultimap is a hamt with values that are also a hamt but are of the set variant.
/// This allows hash sets to be indexable by an address.
pub struct SetMultimap<'a, BS>(pub Map<'a, BS, Cid>);

impl<'a, BS> SetMultimap<'a, BS>
where
    BS: Blockstore,
{
    /// Initializes a new empty SetMultimap.
    pub fn new(bs: &'a BS, hash_algo: Box<dyn HashAlgorithm>) -> Self {
        Self(make_empty_map(bs, HAMT_BIT_WIDTH, hash_algo))
    }

    /// Initializes a SetMultimap from a root Cid.
    pub fn from_root(bs: &'a BS, cid: &Cid, hash_algo: Box<dyn HashAlgorithm>) -> Result<Self, Error> {
        Ok(Self(make_map_with_root(cid, bs, hash_algo)?))
    }

    /// Retrieve root from the SetMultimap.
    #[inline]
    pub fn root(&mut self) -> Result<Cid, Error> {
        self.0.flush()
    }

    /// Puts the DealID in the hash set of the key.
    pub fn put(
        &mut self,
        key: ChainEpoch,
        value: DealID,
		hash_algo1: Box<dyn HashAlgorithm>,
		hash_algo2: Box<dyn HashAlgorithm>,
    ) -> Result<(), Error>
	{
        // Get construct amt from retrieved cid or create new
        let mut set = self
					.get(key, hash_algo1)?
					.unwrap_or_else(||
						Set::new(self.0.store(), hash_algo2)
					);

        set.put(u64_key(value))?;

        // Save and calculate new root
        let new_root = set.root()?;

        // Set hamt node to set new root
        self.0.set(u64_key(key as u64), new_root)?;
        Ok(())
    }

    /// Puts slice of DealIDs in the hash set of the key.
    pub fn put_many(
        &mut self,
        key: ChainEpoch,
        values: &[DealID],
		hash_algo1: Box<dyn HashAlgorithm>,
		hash_algo2: Box<dyn HashAlgorithm>,
    ) -> Result<(), Error> {
        // Get construct amt from retrieved cid or create new
        let mut set = self.get(key, hash_algo1)?.unwrap_or_else(|| Set::new(self.0.store(), hash_algo2));

        for &v in values {
            set.put(u64_key(v))?;
        }

        // Save and calculate new root
        let new_root = set.root()?;

        // Set hamt node to set new root
        self.0.set(u64_key(key as u64), new_root)?;
        Ok(())
    }

    /// Gets the set at the given index of the `SetMultimap`
    #[inline]
    pub fn get(
        &self,
        key: ChainEpoch,
        hash_algo: Box<dyn HashAlgorithm>,
    ) -> Result<Option<Set<'a, BS>>, Error> {
        match self.0.get(&u64_key(key as u64))? {
            Some(cid) => Ok(Some(Set::from_root(*self.0.store(), cid, hash_algo)?)),
            None => Ok(None),
        }
    }

    /// Removes a DealID from a key hash set.
    #[inline]
    pub fn remove(
        &mut self,
        key: ChainEpoch,
        v: DealID,
		hash_algo: Box<dyn HashAlgorithm>,
    ) -> Result<(), Error> {
        // Get construct amt from retrieved cid and return if no set exists
        let mut set = match self.get(key, hash_algo)? {
            Some(s) => s,
            None => return Ok(()),
        };

        set.delete(u64_key(v).borrow())?;

        // Save and calculate new root
        let new_root = set.root()?;
        self.0.set(u64_key(key as u64), new_root)?;
        Ok(())
    }

    /// Removes set at index.
    #[inline]
    pub fn remove_all(
        &mut self,
        key: ChainEpoch,
    ) -> Result<(), Error> {
        // Remove entry from table
        self.0.delete(&u64_key(key as u64))?;

        Ok(())
    }

    /// Iterates through keys and converts them to a DealID to call a function on each.
    pub fn for_each<F>(
        &self,
        key: ChainEpoch,
        mut f: F,
		hash_algo: Box<dyn HashAlgorithm>,
    ) -> Result<(), Error>
    where
        F: FnMut(DealID) -> Result<(), Error>,
    {
        // Get construct amt from retrieved cid and return if no set exists
        let set = match self.get(key, hash_algo)? {
            Some(s) => s,
            None => return Ok(()),
        };

        set.for_each(|k| {
            let v = parse_uint_key(k)
                .map_err(|e| anyhow::anyhow!("Could not parse key: {:?}, ({})", &k.0, e))?;

            // Run function on all parsed keys
            Ok(f(v)?)
        })
    }
}
