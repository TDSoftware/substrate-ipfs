#![cfg_attr(not(feature = "std"), no_std)]

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
    pub trait TDSIpfsApi {
        fn get_file_url_for_cid(cid_bytes: sp_std::vec::Vec<u8>) -> sp_std::vec::Vec<u8>;
		fn get_file_url_for_meta_data(meta_data: sp_std::vec::Vec<u8>) -> sp_std::vec::Vec<u8>;
    }
}

