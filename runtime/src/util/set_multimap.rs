// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use std::borrow::Borrow;

use cid::Cid;
use fvm_ipld_blockstore::Blockstore;
use fvm_ipld_hamt::{Error, HashAlgorithm};
use fvm_shared::clock::ChainEpoch;
use fvm_shared::deal::DealID;
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
    pub fn new(bs: &'a BS) -> Self {
        Self(make_empty_map(bs, HAMT_BIT_WIDTH))
    }

    /// Initializes a SetMultimap from a root Cid.
    pub fn from_root(bs: &'a BS, cid: &Cid) -> Result<Self, Error> {
        Ok(Self(make_map_with_root(cid, bs)?))
    }

    /// Retrieve root from the SetMultimap.
    #[inline]
    pub fn root(&mut self) -> Result<Cid, Error> {
        self.0.flush()
    }

    /// Puts the DealID in the hash set of the key.
    pub fn put<HA>(
        &mut self,
        key: ChainEpoch,
        value: DealID,
        hash_algo: &mut HA,
    ) -> Result<(), Error>
    where
        HA: HashAlgorithm,
    {
        // Get construct amt from retrieved cid or create new
        let mut set = self.get::<_>(key, hash_algo)?.unwrap_or_else(|| Set::new(self.0.store()));

        set.put::<_>(u64_key(value), hash_algo)?;

        // Save and calculate new root
        let new_root = set.root()?;

        // Set hamt node to set new root
        self.0.set::<_>(u64_key(key as u64), new_root, hash_algo)?;
        Ok(())
    }

    /// Puts slice of DealIDs in the hash set of the key.
    pub fn put_many<HA>(
        &mut self,
        key: ChainEpoch,
        values: &[DealID],
        hash_algo: &mut HA,
    ) -> Result<(), Error>
    where
        HA: HashAlgorithm,
    {
        // Get construct amt from retrieved cid or create new
        let mut set = self.get::<_>(key, hash_algo)?.unwrap_or_else(|| Set::new(self.0.store()));

        for &v in values {
            set.put::<_>(u64_key(v), hash_algo)?;
        }

        // Save and calculate new root
        let new_root = set.root()?;

        // Set hamt node to set new root
        self.0.set::<_>(u64_key(key as u64), new_root, hash_algo)?;
        Ok(())
    }

    /// Gets the set at the given index of the `SetMultimap`
    #[inline]
    pub fn get<HA>(&self, key: ChainEpoch, hash_algo: &mut HA) -> Result<Option<Set<'a, BS>>, Error>
    where
        HA: HashAlgorithm,
    {
        match self.0.get::<_, _>(&u64_key(key as u64), hash_algo)? {
            Some(cid) => Ok(Some(Set::from_root(*self.0.store(), cid)?)),
            None => Ok(None),
        }
    }

    /// Removes a DealID from a key hash set.
    #[inline]
    pub fn remove<HA>(
        &mut self,
        key: ChainEpoch,
        v: DealID,
        hash_algo: &mut HA,
    ) -> Result<(), Error>
    where
        HA: HashAlgorithm,
    {
        // Get construct amt from retrieved cid and return if no set exists
        let mut set = match self.get::<_>(key, hash_algo)? {
            Some(s) => s,
            None => return Ok(()),
        };

        set.delete::<_>(u64_key(v).borrow(), hash_algo)?;

        // Save and calculate new root
        let new_root = set.root()?;
        self.0.set::<_>(u64_key(key as u64), new_root, hash_algo)?;
        Ok(())
    }

    /// Removes set at index.
    #[inline]
    pub fn remove_all<HA>(&mut self, key: ChainEpoch, hash_algo: &mut HA) -> Result<(), Error>
    where
        HA: HashAlgorithm,
    {
        // Remove entry from table
        self.0.delete::<_, _>(&u64_key(key as u64), hash_algo)?;

        Ok(())
    }

    /// Iterates through keys and converts them to a DealID to call a function on each.
    pub fn for_each<F, HA>(
        &self,
        key: ChainEpoch,
        mut f: F,
        hash_algo: &mut HA,
    ) -> Result<(), Error>
    where
        F: FnMut(DealID) -> Result<(), Error>,
        HA: HashAlgorithm,
    {
        // Get construct amt from retrieved cid and return if no set exists
        let set = match self.get::<_>(key, hash_algo)? {
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
