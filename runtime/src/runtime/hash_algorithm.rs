use std::hash::Hasher;

use fvm_shared::runtime::traits::{Hash, HashAlgorithm, HashedKey};

#[derive(Default)]
struct RuntimeHasherWrapper(Vec<u8>);

impl Hasher for RuntimeHasherWrapper
{
    fn finish(&self) -> u64 {
        // u64 hash not used in hamt
        0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }
}
