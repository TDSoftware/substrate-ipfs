use crate::{self as pallet_tds_ipfs_core, mock};
use crate::{mock::*, Error, generate_id};
use frame_support::{assert_noop, assert_ok};
use log::info;
use pallet_tds_ipfs_core::Config;
use std::str;


#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		let test = test_generate_id();
		assert!(test.len() > 0);
	});


}
// fn it_works_for_default_value() {
//   new_test_ext().execute_with(|| {
//     // Dispatch a signed extrinsic.
//     assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
//     // Read pallet storage and assert an expected result.
//     assert_eq!(TemplateModule::something(), Some(42));
//   });
// }

