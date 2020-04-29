use libra_types::vm_error::{sub_status::NFE_LCS_SERIALIZATION_FAILURE, StatusCode, VMStatus};
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    loaded_data::types::FatType,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::{values_impl::Reference, Value},
};
use std::collections::VecDeque;
use vm::errors::VMResult;
use move_core_types::account_address::AccountAddress;


pub fn module_address(
    context: &mut impl NativeContext,
    mut ty_args: Vec<Type>,
    args: VecDeque<Value>,
) -> VMResult<NativeResult> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(args.len() == 0);
    let mut ty_args = context.convert_to_fat_types(ty_args)?;
    let ty_arg = ty_args.pop().unwrap();
    let addr = match ty_arg {
        FatType::Struct(ty) if ty.is_resource => ty.address,
        _ => {
            return Err(VMStatus::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                .with_message(format!("{:?} is not resource", ty_arg)))
        }
    };
    Ok(NativeResult::ok(
        native_gas(
            context.cost_table(),
            NativeCostIndex::MODULE_ADDRESS,
            AccountAddress::LENGTH,
        ),
        vec![Value::address(addr)],
    ))
}
