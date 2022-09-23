use {
    crate::interpreter::{ExecutionState, StatusCode, System, U256},
    fil_actors_runtime::runtime::Runtime,
    fvm_ipld_blockstore::Blockstore,
    fvm_ipld_hamt::HashAlgorithm,
};

#[inline]
pub fn sload<'r, BS, RT>(
    state: &mut ExecutionState,
    platform: &'r mut System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    // where?
    let location = state.stack.pop()?;

    // get from storage and place on stack
    let value = match platform.get_storage(location)? {
        Some(val) => val,
        None => U256::zero(),
    };
    state.stack.push(value)
}

#[inline]
pub fn sstore<'r, BS, RT>(
    state: &mut ExecutionState,
    platform: &'r mut System<'r, BS, RT>,
) -> Result<(), StatusCode>
where
    BS: Blockstore,
    RT: Runtime<BS> + HashAlgorithm,
{
    state.stack.with2(|location, value| {
        let opt_value = if value.is_zero() { None } else { Some(*value) };

        platform.set_storage(*location, opt_value)?;
        Ok(())
    })
}
