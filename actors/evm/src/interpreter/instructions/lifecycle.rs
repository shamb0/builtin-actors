use crate::interpreter::address::Address;
use {
    crate::interpreter::{ExecutionState, StatusCode, System},
    fil_actors_runtime::runtime::Runtime,
    fvm_ipld_blockstore::Blockstore,
    fvm_ipld_hamt::HashAlgorithm,
};

#[inline]
pub fn create<'r, BS, RT>(
    _state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
    _create2: bool,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    todo!()
}

#[inline]
pub fn selfdestruct<'r, BS, RT>(
    state: &mut ExecutionState,
    _system: &'r mut System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    let beneficiary = state.stack.pop()?;
    let beneficiary_addr = Address::try_from(beneficiary)?;
    let id_addr = beneficiary_addr.as_id_address().expect("no support for non-ID addresses yet");
    state.selfdestroyed = Some(id_addr);
    Ok(())
}
