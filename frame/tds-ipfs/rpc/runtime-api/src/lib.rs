#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::{prelude::Box, vec::Vec};

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
    pub trait TDSIpfsApi {
        fn get_file_url(cid_bytes: sp_std::vec::Vec<u8>) -> sp_std::vec::Vec<u8>;
    }
}

