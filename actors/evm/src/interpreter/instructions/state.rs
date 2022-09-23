use crate::U256;
use {
    crate::interpreter::{ExecutionState, StatusCode, System},
    fil_actors_runtime::runtime::Runtime,
    fvm_ipld_blockstore::Blockstore,
    fvm_ipld_hamt::HashAlgorithm,
};

#[inline]
pub fn balance<'r, BS, RT>(
    _state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    todo!("requires syscall")
}

#[inline]
pub fn selfbalance<'r, BS, RT>(
    state: &mut ExecutionState,
    platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    // Returns native FIL balance of the receiver. Value precision is identical to Ethereum, so
    // no conversion needed (atto, 1e18).
    state.stack.push(U256::from(&platform.rt.current_balance()))
}
