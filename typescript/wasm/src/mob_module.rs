#[allow(unused_imports)]
use uniffi_runtime_javascript::{self as js, uniffi as u, IntoJs, IntoRust};
use wasm_bindgen::prelude::wasm_bindgen;
extern "C" {
    fn uniffi_mob_fn_clone_client(
        ptr: u::VoidPointer,
        status_: &mut u::RustCallStatus,
    ) -> u::VoidPointer;
    fn uniffi_mob_fn_free_client(ptr: u::VoidPointer, status_: &mut u::RustCallStatus);
    fn uniffi_mob_fn_constructor_client_new(
        config: u::RustBuffer,
        status_: &mut u::RustCallStatus,
    ) -> u::VoidPointer;
    fn uniffi_mob_fn_constructor_client_new_with_signer(
        config: u::RustBuffer,
        signer: u::VoidPointer,
        status_: &mut u::RustCallStatus,
    ) -> u::VoidPointer;
    fn uniffi_mob_fn_method_client_attach_signer(
        ptr: u::VoidPointer,
        _signer: u::VoidPointer,
        status_: &mut u::RustCallStatus,
    );
    fn uniffi_mob_fn_method_client_execute_contract(
        ptr: u::VoidPointer,
        contract_address: u::RustBuffer,
        msg: u::RustBuffer,
        funds: u::RustBuffer,
        memo: u::RustBuffer,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_mob_fn_method_client_get_account(
        ptr: u::VoidPointer,
        address: u::RustBuffer,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_mob_fn_method_client_get_all_balances(
        ptr: u::VoidPointer,
        address: u::RustBuffer,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_mob_fn_method_client_get_balance(
        ptr: u::VoidPointer,
        address: u::RustBuffer,
        denom: u::RustBuffer,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_mob_fn_method_client_get_chain_id(
        ptr: u::VoidPointer,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_mob_fn_method_client_get_height(
        ptr: u::VoidPointer,
        status_: &mut u::RustCallStatus,
    ) -> u64;
    fn uniffi_mob_fn_method_client_get_tx(
        ptr: u::VoidPointer,
        hash: u::RustBuffer,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_mob_fn_method_client_is_synced(
        ptr: u::VoidPointer,
        status_: &mut u::RustCallStatus,
    ) -> i8;
    fn uniffi_mob_fn_method_client_send(
        ptr: u::VoidPointer,
        to_address: u::RustBuffer,
        amount: u::RustBuffer,
        memo: u::RustBuffer,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_mob_fn_clone_signer(
        ptr: u::VoidPointer,
        status_: &mut u::RustCallStatus,
    ) -> u::VoidPointer;
    fn uniffi_mob_fn_free_signer(ptr: u::VoidPointer, status_: &mut u::RustCallStatus);
    fn uniffi_mob_fn_constructor_signer_from_mnemonic(
        mnemonic: u::RustBuffer,
        address_prefix: u::RustBuffer,
        derivation_path: u::RustBuffer,
        status_: &mut u::RustCallStatus,
    ) -> u::VoidPointer;
    fn uniffi_mob_fn_method_signer_address(
        ptr: u::VoidPointer,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_mob_fn_method_signer_address_prefix(
        ptr: u::VoidPointer,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_mob_fn_method_signer_public_key_hex(
        ptr: u::VoidPointer,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_mob_fn_method_signer_sign_bytes(
        ptr: u::VoidPointer,
        message: u::RustBuffer,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_mob_checksum_method_client_attach_signer() -> u16;
    fn uniffi_mob_checksum_method_client_execute_contract() -> u16;
    fn uniffi_mob_checksum_method_client_get_account() -> u16;
    fn uniffi_mob_checksum_method_client_get_all_balances() -> u16;
    fn uniffi_mob_checksum_method_client_get_balance() -> u16;
    fn uniffi_mob_checksum_method_client_get_chain_id() -> u16;
    fn uniffi_mob_checksum_method_client_get_height() -> u16;
    fn uniffi_mob_checksum_method_client_get_tx() -> u16;
    fn uniffi_mob_checksum_method_client_is_synced() -> u16;
    fn uniffi_mob_checksum_method_client_send() -> u16;
    fn uniffi_mob_checksum_method_signer_address() -> u16;
    fn uniffi_mob_checksum_method_signer_address_prefix() -> u16;
    fn uniffi_mob_checksum_method_signer_public_key_hex() -> u16;
    fn uniffi_mob_checksum_method_signer_sign_bytes() -> u16;
    fn uniffi_mob_checksum_constructor_client_new() -> u16;
    fn uniffi_mob_checksum_constructor_client_new_with_signer() -> u16;
    fn uniffi_mob_checksum_constructor_signer_from_mnemonic() -> u16;
    fn ffi_mob_uniffi_contract_version() -> u32;
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_clone_client(
    ptr: js::VoidPointer,
    f_status_: &mut js::RustCallStatus,
) -> js::VoidPointer {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_clone_client(u::VoidPointer::into_rust(ptr), &mut u_status_)
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_free_client(
    ptr: js::VoidPointer,
    f_status_: &mut js::RustCallStatus,
) {
    let mut u_status_ = u::RustCallStatus::default();
    unsafe { uniffi_mob_fn_free_client(u::VoidPointer::into_rust(ptr), &mut u_status_) };
    f_status_.copy_from(u_status_);
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_constructor_client_new(
    config: js::ForeignBytes,
    f_status_: &mut js::RustCallStatus,
) -> js::VoidPointer {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_constructor_client_new(
            u::RustBuffer::into_rust(config),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_constructor_client_new_with_signer(
    config: js::ForeignBytes,
    signer: js::VoidPointer,
    f_status_: &mut js::RustCallStatus,
) -> js::VoidPointer {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_constructor_client_new_with_signer(
            u::RustBuffer::into_rust(config),
            u::VoidPointer::into_rust(signer),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_client_attach_signer(
    ptr: js::VoidPointer,
    _signer: js::VoidPointer,
    f_status_: &mut js::RustCallStatus,
) {
    let mut u_status_ = u::RustCallStatus::default();
    unsafe {
        uniffi_mob_fn_method_client_attach_signer(
            u::VoidPointer::into_rust(ptr),
            u::VoidPointer::into_rust(_signer),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_client_execute_contract(
    ptr: js::VoidPointer,
    contract_address: js::ForeignBytes,
    msg: js::ForeignBytes,
    funds: js::ForeignBytes,
    memo: js::ForeignBytes,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_client_execute_contract(
            u::VoidPointer::into_rust(ptr),
            u::RustBuffer::into_rust(contract_address),
            u::RustBuffer::into_rust(msg),
            u::RustBuffer::into_rust(funds),
            u::RustBuffer::into_rust(memo),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_client_get_account(
    ptr: js::VoidPointer,
    address: js::ForeignBytes,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_client_get_account(
            u::VoidPointer::into_rust(ptr),
            u::RustBuffer::into_rust(address),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_client_get_all_balances(
    ptr: js::VoidPointer,
    address: js::ForeignBytes,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_client_get_all_balances(
            u::VoidPointer::into_rust(ptr),
            u::RustBuffer::into_rust(address),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_client_get_balance(
    ptr: js::VoidPointer,
    address: js::ForeignBytes,
    denom: js::ForeignBytes,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_client_get_balance(
            u::VoidPointer::into_rust(ptr),
            u::RustBuffer::into_rust(address),
            u::RustBuffer::into_rust(denom),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_client_get_chain_id(
    ptr: js::VoidPointer,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_client_get_chain_id(
            u::VoidPointer::into_rust(ptr),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_client_get_height(
    ptr: js::VoidPointer,
    f_status_: &mut js::RustCallStatus,
) -> js::UInt64 {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_client_get_height(
            u::VoidPointer::into_rust(ptr),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_client_get_tx(
    ptr: js::VoidPointer,
    hash: js::ForeignBytes,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_client_get_tx(
            u::VoidPointer::into_rust(ptr),
            u::RustBuffer::into_rust(hash),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_client_is_synced(
    ptr: js::VoidPointer,
    f_status_: &mut js::RustCallStatus,
) -> js::Int8 {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_client_is_synced(
            u::VoidPointer::into_rust(ptr),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_client_send(
    ptr: js::VoidPointer,
    to_address: js::ForeignBytes,
    amount: js::ForeignBytes,
    memo: js::ForeignBytes,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_client_send(
            u::VoidPointer::into_rust(ptr),
            u::RustBuffer::into_rust(to_address),
            u::RustBuffer::into_rust(amount),
            u::RustBuffer::into_rust(memo),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_clone_signer(
    ptr: js::VoidPointer,
    f_status_: &mut js::RustCallStatus,
) -> js::VoidPointer {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_clone_signer(u::VoidPointer::into_rust(ptr), &mut u_status_)
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_free_signer(
    ptr: js::VoidPointer,
    f_status_: &mut js::RustCallStatus,
) {
    let mut u_status_ = u::RustCallStatus::default();
    unsafe { uniffi_mob_fn_free_signer(u::VoidPointer::into_rust(ptr), &mut u_status_) };
    f_status_.copy_from(u_status_);
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_constructor_signer_from_mnemonic(
    mnemonic: js::ForeignBytes,
    address_prefix: js::ForeignBytes,
    derivation_path: js::ForeignBytes,
    f_status_: &mut js::RustCallStatus,
) -> js::VoidPointer {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_constructor_signer_from_mnemonic(
            u::RustBuffer::into_rust(mnemonic),
            u::RustBuffer::into_rust(address_prefix),
            u::RustBuffer::into_rust(derivation_path),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_signer_address(
    ptr: js::VoidPointer,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_signer_address(
            u::VoidPointer::into_rust(ptr),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_signer_address_prefix(
    ptr: js::VoidPointer,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_signer_address_prefix(
            u::VoidPointer::into_rust(ptr),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_signer_public_key_hex(
    ptr: js::VoidPointer,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_signer_public_key_hex(
            u::VoidPointer::into_rust(ptr),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_mob_fn_method_signer_sign_bytes(
    ptr: js::VoidPointer,
    message: js::ForeignBytes,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_mob_fn_method_signer_sign_bytes(
            u::VoidPointer::into_rust(ptr),
            u::RustBuffer::into_rust(message),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_client_attach_signer() -> js::UInt16 {
    uniffi_mob_checksum_method_client_attach_signer().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_client_execute_contract() -> js::UInt16 {
    uniffi_mob_checksum_method_client_execute_contract().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_client_get_account() -> js::UInt16 {
    uniffi_mob_checksum_method_client_get_account().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_client_get_all_balances() -> js::UInt16 {
    uniffi_mob_checksum_method_client_get_all_balances().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_client_get_balance() -> js::UInt16 {
    uniffi_mob_checksum_method_client_get_balance().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_client_get_chain_id() -> js::UInt16 {
    uniffi_mob_checksum_method_client_get_chain_id().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_client_get_height() -> js::UInt16 {
    uniffi_mob_checksum_method_client_get_height().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_client_get_tx() -> js::UInt16 {
    uniffi_mob_checksum_method_client_get_tx().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_client_is_synced() -> js::UInt16 {
    uniffi_mob_checksum_method_client_is_synced().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_client_send() -> js::UInt16 {
    uniffi_mob_checksum_method_client_send().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_signer_address() -> js::UInt16 {
    uniffi_mob_checksum_method_signer_address().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_signer_address_prefix() -> js::UInt16 {
    uniffi_mob_checksum_method_signer_address_prefix().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_signer_public_key_hex() -> js::UInt16 {
    uniffi_mob_checksum_method_signer_public_key_hex().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_method_signer_sign_bytes() -> js::UInt16 {
    uniffi_mob_checksum_method_signer_sign_bytes().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_constructor_client_new() -> js::UInt16 {
    uniffi_mob_checksum_constructor_client_new().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_constructor_client_new_with_signer() -> js::UInt16 {
    uniffi_mob_checksum_constructor_client_new_with_signer().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_mob_checksum_constructor_signer_from_mnemonic() -> js::UInt16 {
    uniffi_mob_checksum_constructor_signer_from_mnemonic().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_ffi_mob_uniffi_contract_version() -> js::UInt32 {
    ffi_mob_uniffi_contract_version().into_js()
}
