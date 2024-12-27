use crate::{mock::*, *};
use polkadot_sdk::frame_support::{assert_ok, assert_err};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		assert!(true);
	});
}