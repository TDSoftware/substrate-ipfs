use frame_support::{assert_ok};
use crate::{mock::*, IpfsCommand, TypeEquality};

#[test]
fn test_generate_id() {
	ExtBuilder::default().build_and_execute(|| {
		let test = mock_generate_id();
		assert!(test.len() > 0);
	});
}

#[test]
fn test_addresses_to_utf8_safe_bytes() {
	ExtBuilder::default().build_and_execute(|| {
		let result = mock_addresses_to_utf8_safe_bytes("test/bytes/öäüß");
		log::debug!("{:?}", result);

		assert!(result.len() > 0);
		let data = [116, 101, 115, 116, 47, 98, 121, 116, 101, 115, 47, 195, 182, 195, 164, 195, 188, 195, 159].to_vec();
		assert_eq!(result, data);

	});
}

#[test]
fn test_multiple_bytes_to_utf8_safe_bytes() {
	ExtBuilder::default().build_and_execute(|| {
		let mut test_data = Vec::<Vec<u8>>::new();
		test_data.push("Test 1".as_bytes().to_vec());
		test_data.push("2 tseT".as_bytes().to_vec());

		let result = mock_multiple_bytes_to_utf8_safe_bytes(test_data);
		log::debug!("{:?}", result);

		assert!(result.len() > 0);
		assert_eq!(result, [84, 101, 115, 116, 32, 49, 44, 32, 50, 32, 116, 115, 101, 84]);
	});
}

#[test]
fn test_connect() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {
		let result = mock_connect_to_localhost();
		assert_ok!(result);
	});
}

#[test]
fn test_disconnect() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {
		let mut result = mock_connect_to_localhost();
		assert_ok!(result);

		result = mock_disconnect_from_localhost();
		println!("{:?}", result);
		assert_ok!(result);
	});
}

#[test]
fn test_add_bytes() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {
		let mut result = mock_connect_to_localhost();
		assert_ok!(result);

		result = mock_add_bytes("Hello IPFS");
		println!("{:?}", &result);
		assert_ok!(&result);
	});
}

#[test]
fn test_cat_bytes() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {
		let mut result = mock_connect_to_localhost();
		assert_ok!(result);

		result = mock_cat_bytes("QmbWqxBEKC3P8tqsKc98xmWNzrzDtRLMiMPL8wBuTGsMnR");
		println!("{:?}", &result);
		assert_ok!(&result);
	});
}


#[test]
fn test_ipfs_command_type_equality() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {
		let cmd_add_bytes_one = IpfsCommand::AddBytes(1);
		let cmd_add_bytes_two = IpfsCommand::AddBytes(0);
		let cmd_cat_bytes = IpfsCommand::CatBytes(vec![3,4,6,8,9]);

		let cmp_add_bytes = cmd_add_bytes_one.eq_type(&cmd_add_bytes_two);
		assert!(cmp_add_bytes);

		let cmp_add_cat_bytes = cmd_add_bytes_two.eq_type(&cmd_cat_bytes) == false;
		assert!(cmp_add_cat_bytes);
	});
}
