#![cfg_attr(not(feature = "std"), no_std)]

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait TemplateApi {
		fn get_value() -> u32;
	}
}
