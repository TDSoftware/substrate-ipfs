use codec::{Encode, Decode};
use frame_support::{assert_ok};
use sp_runtime::traits::BlockNumberProvider;
use crate::{mock::*, storage::{self, OffchainStorageData}, IpfsCommand, TypeEquality};

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

#[test]
fn test_offchain_storage_data() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {

		let data = vec![1,2,3,];
		let test_meta = vec![6, 1, 2, 3, 4];
		let data_1 = OffchainStorageData::new(data, test_meta);
		let encode = data_1.encode();

		let decode = OffchainStorageData::decode(&mut &*encode);
		assert_ok!(&decode);

		let decoded_obj = decode.expect("expected OffchainStorageData");
		assert_eq!(decoded_obj.data, data_1.data);
	});
}


#[test]
fn test_set_offchain_data() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {
		let block_number = System::current_block_number();
		let test_data = vec![6, 1, 2, 3, 4];
		let test_meta = vec![6, 1, 2, 3, 4];

		storage::set_offchain_data::<Test>(block_number, test_data.clone(), test_meta.clone(), true);
		// no assertions here, but set_offchain_data would panic in case something went wrong
	});
}

#[test]
fn test_offchain_data() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {
		let block_number = System::current_block_number();
		let test = vec![6,1,2,3,4];
		let test_meta = vec![6, 1, 2, 3, 4];

		storage::set_offchain_data::<Test>(block_number, test.clone(), test_meta.clone(), true);
		let offchain_data = storage::offchain_data::<Test>(block_number);

		assert_ok!(&offchain_data);
		let data = offchain_data.expect("expected bytes");
		let bytes = data.data;

		println!("bytes = {:?}", bytes);
		assert_eq!(bytes, test);
		assert_eq!(data.meta_data, bytes);
	});
}

#[test]
fn test_offchain_data_key() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {
		let block_number_one = System::current_block_number();
		let key_one = storage::offchain_data_key::<Test>(block_number_one);

		assert_eq!(key_one.is_empty(), false);
		assert_ne!(storage::OFFCHAIN_KEY_PREFIX.len(), key_one.len());

		let block_number_two = System::current_block_number() + 1;
		let key_two = storage::offchain_data_key::<Test>(block_number_two);

		assert_eq!(key_one.is_empty(), false);
		assert_ne!(storage::OFFCHAIN_KEY_PREFIX.len(), key_two.len());
		assert_ne!(key_one, key_two);
	});
}

#[test]
fn test_set_offchain_storage_data_for_block_number() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {
		let data = vec![1,2,3,4,5];
		let storage_data = storage::OffchainStorageData::new(data, vec![]);

		let block_number = System::current_block_number();
		storage::set_offchain_storage_data::<Test>(block_number, &storage_data, true);
		// no assertions here, but set_offchain_data would panic in case something went wrong
	});
}

#[test]
fn test_offchain_storage_data_for_block_number() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {

		let data = vec![1,2,3,4,5];
		let storage_data = storage::OffchainStorageData::new(data, vec![]);

		let block_number = System::current_block_number();
		storage::set_offchain_storage_data::<Test>(block_number, &storage_data, true);

		let result = storage::offchain_storage_data_for_block_number::<Test>(block_number);
		assert_ok!(&result);

		if let Ok(Some(offchain_data_from_storage)) = result {
			assert_eq!(offchain_data_from_storage.data, storage_data.data)
		}
		else {
			assert!(false)
		}
	});
}

#[test]
fn test_set_offchain_storage_data_for_key() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {
		let data = vec![1,2,3,4,5];
		let key = b"test_key".to_vec();

		let storage_data = storage::OffchainStorageData::new(data, vec![]);


		storage::set_offchain_storage_data_for_key::<Test>(&key, &storage_data, true);
		// no assertions here, but set_offchain_data would panic in case something went wrong
	});
}

#[test]
fn test_offchain_storage_data_for_key() {
	ExtBuilder::default().build_and_execute_for_offchain(|| {
		let data = vec![1,2,3,4,5];
		let key = b"test_key".to_vec();

		let storage_data = storage::OffchainStorageData::new(data, vec![]);
		storage::set_offchain_storage_data_for_key::<Test>(&key, &storage_data, true);

		let result = storage::offchain_storage_data_for_key(&key);
		assert_ok!(&result);

		if let Ok(Some(offchain_data_from_storage)) = result {
			assert_eq!(offchain_data_from_storage.data, storage_data.data)
		}
		else {
			assert!(false)
		}
	});
}
