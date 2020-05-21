use crate::error::Error;
use alloc::string::String;
use casperlabs_contract::{
    args_parser::ArgsParser,
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casperlabs_types::{
    account::PublicKey,
    bytesrepr::{Error as ApiError, FromBytes, ToBytes},
    CLTyped, ContractRef, Key,
};
use core::convert::TryInto;

const ADMIN_KEY: &str = "admin_account";
const INIT_FLAG: &str = "init_flag";

pub fn deploy_code_and_init<A>(fn_name: &str, key_name: &str, init_args: A)
where
    A: ArgsParser,
{
    deploy(fn_name, key_name, Some(init_args));
}

pub fn deploy_code(fn_name: &str, key_name: &str) {
    deploy::<()>(fn_name, key_name, None);
}

fn deploy<A>(fn_name: &str, key_name: &str, init_args: Option<A>)
where
    A: ArgsParser,
{
    let contract_ref: ContractRef = storage::store_function_at_hash(fn_name, Default::default());

    if let Some(args) = init_args {
        runtime::call_contract::<_, ()>(contract_ref.clone(), args);
    }

    let key: Key = contract_ref.into();
    set_key(key_name, key);
}

pub fn key<T: FromBytes + CLTyped>(name: &str) -> T {
    let key = runtime::get_key(name)
        .unwrap_or_revert_with(Error::MissingKey)
        .try_into()
        .unwrap_or_revert_with(Error::UnexpectedType);
    storage::read(key)
        .unwrap_or_revert_with(Error::MissingKey)
        .unwrap_or_revert_with(Error::UnexpectedType)
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

pub fn destination_contract() -> ContractRef {
    let (_, hash): (String, [u8; 32]) = get_arg(0);
    ContractRef::Hash(hash)
}

pub fn get_arg<T: CLTyped + FromBytes>(i: u32) -> T {
    runtime::get_arg(i)
        .unwrap_or_revert_with(Error::missing_argument(i))
        .unwrap_or_revert_with(Error::invalid_argument(i))
}

pub fn method_name() -> String {
    let maybe_argument: Result<String, ApiError> =
        runtime::get_arg(0).unwrap_or_revert_with(Error::missing_argument(0));
    match maybe_argument {
        Ok(method) => method,
        Err(_) => {
            let (method, _): (String, [u8; 32]) = get_arg(0);
            method
        }
    }
}

pub fn init_or_handle<F, G>(init: F, handle: G)
where
    F: Fn() -> Result<(), Error>,
    G: Fn() -> Result<(), Error>,
{
    if runtime::has_key(INIT_FLAG) {
        handle().unwrap_or_revert();
    } else {
        init().unwrap_or_revert();
        set_key(INIT_FLAG, true);
    }
}

pub fn set_admin_account(admin: PublicKey) {
    set_key(ADMIN_KEY, admin);
}

pub fn assert_admin() {
    let admin: PublicKey = key(ADMIN_KEY);
    let caller = runtime::get_caller();
    if admin != caller {
        runtime::revert(Error::NotTheAdminAccount);
    }
}
