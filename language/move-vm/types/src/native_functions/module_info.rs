use crate::{
    loaded_data::types::FatType,
    native_functions::dispatch::{native_gas, NativeResult},
    values::Value,
};
use libra_types::account_address::AccountAddress;
use libra_types::vm_error::{StatusCode, VMStatus};
use move_core_types::gas_schedule::{CostTable, NativeCostIndex};
use std::collections::VecDeque;
use vm::errors::VMResult;

pub fn module_address(
    mut ty_args: Vec<FatType>,
    args: VecDeque<Value>,
    cost_table: &CostTable,
) -> VMResult<NativeResult> {
    if ty_args.len() != 1 {
        let msg = format!(
            "Wrong number of type arguments for module_address. Expected 1, but found {}",
            ty_args.len()
        );
        return Err(VMStatus::new(StatusCode::UNREACHABLE).with_message(msg));
    }
    if args.len() != 0 {
        let msg = format!(
            "Wrong number of arguments for module_address. Expected 0, but found {}",
            args.len()
        );
        return Err(VMStatus::new(StatusCode::UNREACHABLE).with_message(msg));
    }
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
            cost_table,
            NativeCostIndex::MODULE_ADDRESS,
            AccountAddress::LENGTH,
        ),
        vec![Value::address(addr)],
    ))
}
