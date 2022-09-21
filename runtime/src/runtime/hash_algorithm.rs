use std::hash::Hasher;
use std::marker::PhantomData;

use fvm_shared::runtime::traits::{Hash, HashAlgorithm, HashedKey};
use fvm_ipld_blockstore::Blockstore;
use crate::{Runtime};

pub struct RuntimeHashAlgoWrap<'a, B: Blockstore, RT: Runtime<B>>(Box<&'a RT>, PhantomData<B>);

impl<'a, B, RT> HashAlgorithm for RuntimeHashAlgoWrap<'a, B, RT>
where
    B: Blockstore,
	RT: Runtime<B>
{
    fn rt_hash(&self, key: &dyn Hash) -> HashedKey {
		let mut hasher = RuntimeHasherWrapper::default();
        key.hash(&mut hasher);
        self.0.hash_finalize(&hasher.0)
    }
}

// impl<'a, B, RT> From<&'a RT> for RuntimeHashAlgoWrap<'a, B, RT>
// where
// 	B: Blockstore,
// 	RT: Runtime<B>
// {
// 	fn from(rt: &'a RT) -> Self {
// 		Self(Box::new(rt), Default::default())
// 	}
// }

impl<'a, B, RT> RuntimeHashAlgoWrap<'a, B, RT>
where
	B: Blockstore,
	RT: Runtime<B>
{
	pub fn from_rt(rt: &'a RT) -> Self {
		Self(Box::new(rt), Default::default())
	}
}

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
