use crate::{Error, mock::*};
use frame_support::{assert_err, assert_noop, assert_ok};

#[test]
fn it_works_for_create_claim() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_claim(Origin::signed(1),vec![0,1]));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::proofs(vec![0,1]), (1,0));
	});
}

#[test]
fn it_works_for_invoke_claim() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_claim(Origin::signed(1),vec![0,1]));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::proofs(vec![0,1]), (1,0));
		assert_ok!(TemplateModule::revoke_claim(Origin::signed(1),vec![0,1]));
		assert_eq!(TemplateModule::proofs(vec![0,1]), (0,0));
	});
}

#[test]
fn it_works_for_change_onwer_claim() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_claim(Origin::signed(1),vec![0,1]));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::proofs(vec![0,1]), (1,0));
		assert_ok!(TemplateModule::change_owner_claim(Origin::signed(1),vec![0,1],2));
		assert_eq!(TemplateModule::proofs(vec![0,1]), (2,0));
	});
}

#[test]
fn it_works_for_over_length_claim() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_err!(TemplateModule::create_claim(Origin::signed(1),vec![0,1,2,3,4,5,6,7,8,9]),"OverLength");
		// assert_ok!(TemplateModule::create_claim(Origin::signed(1),vec![0,1,2,3,4,5,6,7,8,9]));
		// Read pallet storage and assert an expected result.
	});
}