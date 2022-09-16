use {
    crate::interpreter::{ExecutionState, StatusCode, System},
    fil_actors_runtime::runtime::Runtime,
    fvm_ipld_blockstore::Blockstore,
	fvm_ipld_hamt::{HashAlgorithm},
};

#[inline]
pub fn log<'r, BS, RT>(
    _state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
    _num_topics: usize,
) -> Result<(), StatusCode>
where
	BS: Blockstore,
	RT: Runtime<BS> + HashAlgorithm
{
    todo!()
}
