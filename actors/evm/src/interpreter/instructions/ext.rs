use {
    crate::interpreter::{ExecutionState, StatusCode, System},
    fil_actors_runtime::runtime::Runtime,
    fvm_ipld_blockstore::Blockstore,
	fvm_ipld_hamt::{HashAlgorithm},
};

#[inline]
pub fn extcodesize<'r, BS, RT>(
    _state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
	BS: Blockstore,
	RT: Runtime<BS> + HashAlgorithm
{
    // TODO
    //  1. call actor::get_actor_code_cid
    //  2. check that it matches our code CID (it's an EVM actor)
    //  3. call GetEvmBytecode method, returns the CID of the EVM bytecode block
    //  4. open the block
    //  5. return the length
    todo!()
}

pub fn extcodehash<'r, BS, RT>(
    _state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
	BS: Blockstore,
	RT: Runtime<BS> + HashAlgorithm
{
    // TODO
    todo!();
}

pub fn extcodecopy<'r, BS, RT>(
    _state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
	BS: Blockstore,
	RT: Runtime<BS> + HashAlgorithm
{
    todo!();
}
