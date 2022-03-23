use crate::{Error, mock::*, TotalSupply};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_mock_build_genesis() {
	new_test_ext_with_event().execute_with(|| {
        assert_eq!(get_test_total_supply(), PalletErc20::get_total_supply());
        BALANCES.iter().for_each(|(acc, bal)|{
            assert_eq!(*bal, PalletErc20::get_balance(acc));
        })
	});
}