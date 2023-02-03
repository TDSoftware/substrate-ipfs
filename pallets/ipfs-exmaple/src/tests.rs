use crate::{mock::*, DataCommand, Error};
use frame_support::{assert_noop, assert_ok};
use log::info;
use std::str;

/*#[test]
fn it_works_for_default_value() {
  new_test_ext().execute_with(|| {
    // Dispatch a signed extrinsic.
    assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
    // Read pallet storage and assert an expected result.
    assert_eq!(TemplateModule::something(), Some(42));
  });
}
*/

#[test]
fn it_expects_ipfs_connect_to_add_a_connection() {
  let localhost = vec![127, 0, 0, 1];

  new_test_ext().execute_with(|| {
    assert_ok!(RsIpfs::ipfs_connect(Origin::signed(1), localhost));
    // println!("Value in commands: {:?}", RsIpfs::commands());
    assert_eq!(RsIpfs::commands().unwrap().len(), 1);
  });
}
#[test]
fn it_expects_ipfs_connect_to_have_multiple_connections() {
  let localhost = vec![127, 0, 0, 1];
  let another_host = vec![1, 1, 1, 1];

  new_test_ext().execute_with(|| {
    assert_ok!(RsIpfs::ipfs_connect(Origin::signed(1), localhost));
    assert_eq!(RsIpfs::commands().unwrap().len(), 1);

    assert_ok!(RsIpfs::ipfs_connect(Origin::signed(1), another_host));
    assert_eq!(RsIpfs::commands().unwrap().len(), 2);
  });
}

#[test]
fn it_expects_ipfs_disconnect_to_add_to_the_queue() {
  let localhost = vec![127, 0, 0, 1];
  vec![1, 1, 1, 1];

  new_test_ext().execute_with(|| {
    assert_ok!(RsIpfs::ipfs_disconnect(Origin::signed(1), localhost));
    assert_eq!(RsIpfs::commands().unwrap().len(), 1)
  });
}

#[test]
fn it_expects_ipfs_disconnect_to_be_able_to_process_multiple_disconnections() {
  let localhost = vec![127, 0, 0, 1];
  let another_host = vec![1, 1, 1, 1];

  new_test_ext().execute_with(|| {
    assert_ok!(RsIpfs::ipfs_disconnect(Origin::signed(1), localhost));
    assert_eq!(RsIpfs::commands().unwrap().len(), 1);

    assert_ok!(RsIpfs::ipfs_disconnect(Origin::signed(1), another_host));
    assert_eq!(RsIpfs::commands().unwrap().len(), 2);
  });
}

/*#[test]
fn it_expects_ipfs_disconnect_to_remove_awaiting_connections() {
  let localhost_connection = vec![127, 0, 0, 1];
  let localhost_disconnection = localhost_connection.clone();

  new_test_ext().execute_with(|| {
    assert_ok!(RsIpfs::ipfs_connect(Origin::signed(1), localhost_connection));
    assert_eq!(RsIpfs::commands().unwrap().len(), 1);

    assert_ok!(RsIpfs::ipfs_disconnect(Origin::signed(1), localhost_disconnection));
    assert_eq!(RsIpfs::commands().unwrap().len(), 0);
  });
}
*/

#[test]
fn it_expects_ipfs_add_bytes_to_store_bytes() {
  let message = "Hello world";
  let message_as_bytes = message.as_bytes().to_vec();

  new_test_ext().execute_with(|| {
    assert_ok!(RsIpfs::ipfs_add_bytes(Origin::signed(1), message_as_bytes));
    let queue = RsIpfs::data_queue().unwrap();
    assert_eq!(queue.len(), 1);

    let stored_bytes = queue[0].clone();

    match &stored_bytes {
      DataCommand::AddBytes(value) => {
        assert_eq!(str::from_utf8(value).unwrap(), message);
      },
      _ => {
        panic!("unexpected data type in DataQueue")
      },
    }
  })
}
