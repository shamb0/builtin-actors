// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use {
    crate::runtime::Runtime,
    fvm_ipld_blockstore::Blockstore,
    fvm_ipld_hamt::{Hash, HashAlgorithm, HashedKey},
    std::hash::Hasher,
    std::marker::PhantomData,
};

pub struct Sha2HasherWrapper<'r, BS, RT>
where
    BS: Blockstore,
    RT: Runtime<BS>,
{
    pub rt: &'r RT,
    data: Vec<u8>,
    stub: PhantomData<BS>,
}

impl<'r, BS, RT> Sha2HasherWrapper<'r, BS, RT>
where
    BS: Blockstore,
    RT: Runtime<BS>,
{
    pub fn new(rt: &'r RT) -> Self {
        Self { rt, data: vec![], stub: Default::default() }
    }

    pub fn finalize(&mut self) -> HashedKey {
        let mut rval: HashedKey = Default::default();

        rval.copy_from_slice(
            &self.rt.hash(fvm_shared::crypto::hash::SupportedHashes::Sha2_256, &self.data),
        );

        self.data = vec![];

        rval
    }
}

impl<'r, BS, RT> Hasher for Sha2HasherWrapper<'r, BS, RT>
where
    BS: Blockstore,
    RT: Runtime<BS>,
{
    fn finish(&self) -> u64 {
        // u64 hash not used in hamt
        0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }
}

impl<'r, BS, RT> HashAlgorithm for Sha2HasherWrapper<'r, BS, RT>
where
    BS: Blockstore,
    RT: Runtime<BS>,
{
    fn rt_hash<X>(&mut self, key: &X) -> HashedKey
    where
        X: Hash + ?Sized,
    {
        key.hash(self);
        self.finalize().into()
    }
}
