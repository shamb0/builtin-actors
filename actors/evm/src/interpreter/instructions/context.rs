use crate::interpreter::address::Address;
use {
    crate::interpreter::{ExecutionState, StatusCode, System, U256},
    fil_actors_runtime::runtime::Runtime,
    fvm_ipld_blockstore::Blockstore,
    fvm_ipld_hamt::HashAlgorithm,
};

#[inline]
pub fn blockhash<'r, BS, RT>(
    _state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    todo!("requires the client passing down the inclusion tipset hash")
}

#[inline]
pub fn caller<'r, BS, RT>(
    state: &mut ExecutionState,
    platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    let id = platform.rt.message().caller().id().unwrap();
    state.stack.push(Address::from_id(id).as_evm_word())
}

#[inline]
pub fn address<'r, BS, RT>(
    state: &mut ExecutionState,
    platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    let id = platform.rt.message().receiver().id().unwrap();
    state.stack.push(Address::from_id(id).as_evm_word())
}

#[inline]
pub fn origin<'r, BS, RT>(
    state: &mut ExecutionState,
    platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    let id = platform.rt.message().origin().id().unwrap();
    state.stack.push(Address::from_id(id).as_evm_word())
}

#[inline]
pub fn call_value<'r, BS, RT>(
    state: &mut ExecutionState,
    platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    state.stack.push(U256::from(&platform.rt.message().value_received()))
}

#[inline]
pub fn coinbase<'r, BS, RT>(
    state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    // TODO do we want to return the zero ID address, or just a plain 0?
    state.stack.push(U256::zero())
}

#[inline]
pub fn gas_price<'r, BS, RT>(
    _state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    todo!("should return priority fee (needs syscall) + basefee")
}

#[inline]
pub fn timestamp<'r, BS, RT>(
    _state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    todo!("should return the timestamp from the block header (requires syscall and FFI change)")
}

#[inline]
pub fn block_number<'r, BS, RT>(
    state: &mut ExecutionState,
    platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    state.stack.push(U256::from(platform.rt.curr_epoch()))
}

#[inline]
pub fn difficulty<'r, BS, RT>(
    state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    state.stack.push(U256::zero())
}

#[inline]
pub fn gas_limit<'r, BS, RT>(
    _state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    todo!("requires a syscall")
}

#[inline]
pub fn chain_id<'r, BS, RT>(
    _state: &mut ExecutionState,
    _platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    todo!("requires chain ID registration and configuration in the client")
}

#[inline]
pub fn base_fee<'r, BS, RT>(
    state: &mut ExecutionState,
    platform: &'r System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    state.stack.push(U256::from(&platform.rt.base_fee()))
}
