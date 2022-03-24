use crate::{Error, mock::*, DEFAULT_DECIMALS};
use frame_support::{assert_ok, assert_noop};

type RuntimeError = Error<Test>;

#[test]
fn it_works_for_mock_build_genesis() {
    new_test_ext().execute_with(|| {
        assert_eq!(get_test_total_supply(), PalletErc20::get_total_supply());
        BALANCES.iter().for_each(|(acc, bal)|{
                assert_eq!(*bal, PalletErc20::get_balance(acc));
        });
        assert_eq!(DEFAULT_DECIMALS, PalletErc20::get_decimals());
        assert_eq!(get_test_token_name(), PalletErc20::get_name());
        assert_eq!(get_test_token_sym(), PalletErc20::get_symbol());
    });
}

#[test]
fn it_works_transfer_token() {
    new_test_ext().execute_with(|| {
        let sender_acc = BALANCES[0].0;
        let reciever_acc = BALANCES[3].0;
        let amount = 500;

        let sender_bal_before = PalletErc20::get_balance(sender_acc);
        let reciever_bal_before = PalletErc20::get_balance(reciever_acc);
        let transfer_result = PalletErc20::transfer(Origin::signed(sender_acc), reciever_acc, amount);
        let event = last_event().unwrap();
        let check_event = Event::pallet_erc20(crate::Event::Transfer(sender_acc, reciever_acc, amount));

        assert_ok!(transfer_result, ().into());
        assert_eq!(sender_bal_before - amount, PalletErc20::get_balance(sender_acc));
        assert_eq!(reciever_bal_before + amount, PalletErc20::get_balance(reciever_acc));
        assert_eq!(check_event, event);
    });
}

#[test]
fn it_fails_transfer_token_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let sender_acc = BALANCES[2].0;
        let reciever_acc = BALANCES[3].0;

        let sender_bal_before = PalletErc20::get_balance(sender_acc);
        let amount = 1 + sender_bal_before;
        let reciever_bal_before = PalletErc20::get_balance(reciever_acc);
        let transfer_result = PalletErc20::transfer(Origin::signed(sender_acc), reciever_acc, amount);

        assert_noop!(transfer_result, RuntimeError::TransferAmountExceedsBalance);
        assert_eq!(sender_bal_before, PalletErc20::get_balance(sender_acc));
        assert_eq!(reciever_bal_before, PalletErc20::get_balance(reciever_acc));
    });
}

#[test]
fn it_works_change_allowance() {
    new_test_ext().execute_with(|| {
        let sender_acc = BALANCES[2].0;
        let donor_acc = BALANCES[0].0;
        let value = 5000;
        let derc_value = 1000;

        let alow_before = PalletErc20::get_allowance(donor_acc, sender_acc);
        let incr_allow_res = PalletErc20::increase_allowance(Origin::signed(donor_acc), sender_acc, value);
        let alow_after_inc = PalletErc20::get_allowance(donor_acc, sender_acc);

        let event_after_incr = last_event().unwrap();
        let check_event_after_incr  = Event::pallet_erc20(crate::Event::Approval(donor_acc, sender_acc, value));

        let decr_allow_res = PalletErc20::decrease_allowance(Origin::signed(donor_acc), sender_acc, derc_value);
        let alow_after_dec = PalletErc20::get_allowance(donor_acc, sender_acc);

        let event_after_decr = last_event().unwrap();
        let check_event_after_decr  = Event::pallet_erc20(crate::Event::Approval(donor_acc, sender_acc, value - derc_value));

        assert_ok!(incr_allow_res, ().into());
        assert_ok!(decr_allow_res, ().into());
        assert_eq!(0, alow_before);
        assert_eq!(value, alow_after_inc);
        assert_eq!(value - derc_value, alow_after_dec);
        assert_eq!(check_event_after_decr, event_after_decr);
        assert_eq!(check_event_after_incr, event_after_incr);
    });
}

#[test]
fn it_fails_decrease_allowance() {
    new_test_ext().execute_with(|| {
        let sender_acc = BALANCES[2].0;
        let donor_acc = BALANCES[0].0;
        let value = 5000;

        // try to increase to invorrect value
        let alow_before = PalletErc20::get_allowance(donor_acc, sender_acc);
        // increase to correct value
        let _ = PalletErc20::increase_allowance(Origin::signed(donor_acc), sender_acc, value);
        let alow_after_inc = PalletErc20::get_allowance(donor_acc, sender_acc);

        // try to decrease to incorrect value
        let decr_allow_incorrect_res = PalletErc20::decrease_allowance(Origin::signed(donor_acc), sender_acc, value + 1);
        let alow_after_dec = PalletErc20::get_allowance(donor_acc, sender_acc);

        assert_noop!(decr_allow_incorrect_res, RuntimeError::DecreasedAllowanceBelowZero);
        assert_eq!(0, alow_before);
        assert_eq!(value, alow_after_inc);
        assert_eq!(value, alow_after_dec);
    });
}


#[test]
fn it_works_transfer_from() {
    new_test_ext().execute_with(|| {
        let sender_acc = BALANCES[2].0;
        let reciever_acc = BALANCES[3].0;
        let donor_acc = BALANCES[0].0;
        let value = 5000;
        let transfer_value = 3000;

        let _ = PalletErc20::increase_allowance(Origin::signed(donor_acc), sender_acc, value);
        let alow_after_inc = PalletErc20::get_allowance(donor_acc, sender_acc);

        let transfer_from_result = PalletErc20::transfer_from(Origin::signed(sender_acc), donor_acc, reciever_acc, transfer_value);
        let allow_after_transfer = PalletErc20::get_allowance(donor_acc, sender_acc);

        assert_ok!(transfer_from_result, ().into());
        assert_eq!(transfer_value, PalletErc20::get_balance(reciever_acc));
        assert_eq!(value, alow_after_inc);
        assert_eq!(value - transfer_value, allow_after_transfer);
    });
}

#[test]
fn it_fails_transfer_from() {
    new_test_ext().execute_with(|| {
        let sender_acc = BALANCES[2].0;
        let reciever_acc = BALANCES[3].0;
        let donor_acc = BALANCES[0].0;
        let value = 5000;

        let _ = PalletErc20::increase_allowance(Origin::signed(donor_acc), sender_acc, value);
        let alow_after_inc = PalletErc20::get_allowance(donor_acc, sender_acc);

        // FAILS when not enough allowance
        let transfer_from_result = PalletErc20::transfer_from(Origin::signed(sender_acc), donor_acc, reciever_acc, value + 1);
        let allow_after_transfer = PalletErc20::get_allowance(donor_acc, sender_acc);

        let _ = PalletErc20::increase_allowance(Origin::signed(donor_acc), sender_acc, BALANCES[0].1 + 500);
        let transfer_from_result_over_bal = PalletErc20::transfer_from(Origin::signed(sender_acc), donor_acc, reciever_acc, BALANCES[0].1 + 1);

        assert_noop!(transfer_from_result, RuntimeError::InsufficientAllowance);
        assert_noop!(transfer_from_result_over_bal, RuntimeError::TransferAmountExceedsBalance);
        assert_eq!(alow_after_inc, allow_after_transfer);
    });
}


#[test]
fn it_works_mint_tokens() {
    new_test_ext().execute_with(|| {
        let sender_acc = BALANCES[3].0;
        let mint_amount = 500;

        let total_supply_before = PalletErc20::get_total_supply();
        let mint_call = PalletErc20::_mint(sender_acc, mint_amount);

        assert_ok!(mint_call, ().into());
        assert_eq!(BALANCES[3].1 + mint_amount, PalletErc20::get_balance(sender_acc));
        assert_eq!(total_supply_before + mint_amount, PalletErc20::get_total_supply());
    });
}

#[test]
fn it_works_burn_tokens() {
    new_test_ext().execute_with(|| {
        let sender_acc = BALANCES[2].0;
        let burn_amount = 500;

        let failed_burn_call = PalletErc20::_burn(sender_acc, BALANCES[2].1 + 1);
        let total_supply_before = PalletErc20::get_total_supply();
        let burn_call = PalletErc20::_burn(sender_acc, burn_amount);

        assert_noop!(failed_burn_call, RuntimeError::BurnAmountExceedsBalance);
        assert_ok!(burn_call, ().into());
        assert_eq!(BALANCES[2].1 - burn_amount, PalletErc20::get_balance(sender_acc));
        assert_eq!(total_supply_before - burn_amount, PalletErc20::get_total_supply());
    });
}