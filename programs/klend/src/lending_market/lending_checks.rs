use anchor_lang::{
    accounts::account_loader::AccountLoader,
    err, error,
    prelude::{msg, Context, Pubkey},
    require_eq, require_gte, Key, Result, ToAccountInfo,
};

use crate::{
    handlers::*,
    state::{
        DepositObligationCollateralAccounts, RedeemReserveCollateralAccounts,
        WithdrawObligationCollateralAccounts,
        WithdrawObligationCollateralAndRedeemReserveCollateralAccounts,
    },
    utils::{
        constraints, seeds::BASE_SEED_REFERRER_TOKEN_STATE, FatAccountLoader, PROGRAM_VERSION,
    },
    LendingAction, LendingError, Obligation, ReferrerTokenState, Reserve, ReserveStatus,
};

pub fn borrow_obligation_liquidity_checks(accounts: &BorrowObligationLiquidity) -> Result<()> {
    let borrow_reserve = &accounts.borrow_reserve.load()?;

    if borrow_reserve.liquidity.supply_vault == accounts.user_destination_liquidity.key() {
        msg!(
            "Borrow reserve liquidity supply cannot be used as the destination liquidity provided"
        );
        return err!(LendingError::InvalidAccountInput);
    }

    if borrow_reserve.config.status() == ReserveStatus::Obsolete {
        msg!("Reserve is not active");
        return err!(LendingError::ReserveObsolete);
    }

    if borrow_reserve.version != PROGRAM_VERSION as u64 {
        msg!("Reserve version does not match the program version");
        return err!(LendingError::ReserveDeprecated);
    }

    constraints::token_2022::validate_liquidity_token_extensions(
        &accounts.borrow_reserve_liquidity_mint.to_account_info(),
        &accounts.user_destination_liquidity.to_account_info(),
    )?;

    Ok(())
}

pub fn deposit_obligation_collateral_checks(
    accounts: &DepositObligationCollateralAccounts,
) -> Result<()> {
    let deposit_reserve = &accounts.deposit_reserve.load()?;

    if deposit_reserve.collateral.supply_vault == accounts.user_source_collateral.key() {
        msg!("Deposit reserve collateral supply cannot be used as the source collateral provided");
        return err!(LendingError::InvalidAccountInput);
    }

    if deposit_reserve.config.status() == ReserveStatus::Obsolete {
        msg!("Reserve is not active");
        return err!(LendingError::ReserveObsolete);
    }

    if deposit_reserve.version != PROGRAM_VERSION as u64 {
        msg!("Reserve version does not match the program version");
        return err!(LendingError::ReserveDeprecated);
    }

    Ok(())
}

pub fn deposit_reserve_liquidity_checks(
    accounts: &crate::state::nested_accounts::DepositReserveLiquidityAccounts,
) -> Result<()> {
    let reserve = accounts.reserve.load()?;

    if reserve.liquidity.supply_vault == accounts.user_source_liquidity.key() {
        msg!("Reserve liquidity supply cannot be used as the source liquidity provided");
        return err!(LendingError::InvalidAccountInput);
    }
    if reserve.collateral.supply_vault == accounts.user_destination_collateral.key() {
        msg!("Reserve collateral supply cannot be used as the destination collateral provided");
        return err!(LendingError::InvalidAccountInput);
    }

    if reserve.config.status() == ReserveStatus::Obsolete {
        msg!("Reserve is not active");
        return err!(LendingError::ReserveObsolete);
    }

    if reserve.version != PROGRAM_VERSION as u64 {
        msg!("Reserve version does not match the program version");
        return err!(LendingError::ReserveDeprecated);
    }

    constraints::token_2022::validate_liquidity_token_extensions(
        &accounts.reserve_liquidity_mint.to_account_info(),
        &accounts.user_source_liquidity.to_account_info(),
    )?;

    Ok(())
}

pub fn deposit_reserve_liquidity_and_obligation_collateral_checks(
    accounts: &crate::state::nested_accounts::DepositReserveLiquidityAndObligationCollateralAccounts,
) -> Result<()> {
    let reserve = accounts.reserve.load()?;

    if reserve.liquidity.supply_vault == accounts.user_source_liquidity.key() {
        msg!("Reserve liquidity supply cannot be used as the source liquidity provided");
        return err!(LendingError::InvalidAccountInput);
    }

    if reserve.config.status() == ReserveStatus::Obsolete {
        msg!("Reserve is not active");
        return err!(LendingError::ReserveObsolete);
    }

    if reserve.version != PROGRAM_VERSION as u64 {
        msg!("Reserve version does not match the program version");
        return err!(LendingError::ReserveDeprecated);
    }

    constraints::token_2022::validate_liquidity_token_extensions(
        &accounts.reserve_liquidity_mint.to_account_info(),
        &accounts.user_source_liquidity.to_account_info(),
    )?;

    Ok(())
}

pub fn liquidate_obligation_checks(
    accounts: &LiquidateObligationAndRedeemReserveCollateral,
) -> Result<()> {
    let repay_reserve = accounts.repay_reserve.load()?;
    let withdraw_reserve = accounts.withdraw_reserve.load()?;

    if repay_reserve.liquidity.supply_vault == accounts.user_source_liquidity.key() {
        msg!("Repay reserve liquidity supply cannot be used as the source liquidity provided");
        return err!(LendingError::InvalidAccountInput);
    }
    if repay_reserve.collateral.supply_vault == accounts.user_destination_collateral.key() {
        msg!(
            "Repay reserve collateral supply cannot be used as the destination collateral provided"
        );
        return err!(LendingError::InvalidAccountInput);
    }

    if repay_reserve.version != PROGRAM_VERSION as u64 {
        msg!("Withdraw reserve version does not match the program version");
        return err!(LendingError::ReserveDeprecated);
    }

    if withdraw_reserve.liquidity.supply_vault == accounts.user_source_liquidity.key() {
        msg!("Withdraw reserve liquidity supply cannot be used as the source liquidity provided");
        return err!(LendingError::InvalidAccountInput);
    }
    if withdraw_reserve.collateral.supply_vault == accounts.user_destination_collateral.key() {
        msg!("Withdraw reserve collateral supply cannot be used as the destination collateral provided");
        return err!(LendingError::InvalidAccountInput);
    }

    if withdraw_reserve.version != PROGRAM_VERSION as u64 {
        msg!("Withdraw reserve version does not match the program version");
        return err!(LendingError::ReserveDeprecated);
    }

    constraints::token_2022::validate_liquidity_token_extensions(
        &accounts.repay_reserve_liquidity_mint.to_account_info(),
        &accounts.user_source_liquidity.to_account_info(),
    )?;

    constraints::token_2022::validate_liquidity_token_extensions(
        &accounts.withdraw_reserve_liquidity_mint.to_account_info(),
        &accounts.user_destination_liquidity.to_account_info(),
    )?;

    Ok(())
}

pub fn redeem_reserve_collateral_checks(accounts: &RedeemReserveCollateralAccounts) -> Result<()> {
    let reserve = &accounts.reserve.load()?;

    if reserve.collateral.supply_vault == accounts.user_source_collateral.key() {
        msg!("Reserve collateral supply cannot be used as the source collateral provided");
        return err!(LendingError::InvalidAccountInput);
    }
    if reserve.liquidity.supply_vault == accounts.user_destination_liquidity.key() {
        msg!("Reserve liquidity supply cannot be used as the destination liquidity provided");
        return err!(LendingError::InvalidAccountInput);
    }

    if reserve.version != PROGRAM_VERSION as u64 {
        msg!("Reserve version does not match the program version");
        return err!(LendingError::ReserveDeprecated);
    }

    constraints::token_2022::validate_liquidity_token_extensions(
        &accounts.reserve_liquidity_mint.to_account_info(),
        &accounts.user_destination_liquidity.to_account_info(),
    )?;

    Ok(())
}

pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_checks(
    accounts: &WithdrawObligationCollateralAndRedeemReserveCollateralAccounts,
) -> Result<()> {
    let withdraw_reserve = accounts.withdraw_reserve.load()?;

    if withdraw_reserve.version != PROGRAM_VERSION as u64 {
        msg!("Reserve version does not match the program version");
        return err!(LendingError::ReserveDeprecated);
    }

    if withdraw_reserve.liquidity.supply_vault == accounts.user_destination_liquidity.key() {
        msg!("Reserve liquidity supply cannot be used as the destination liquidity provided");
        return err!(LendingError::InvalidAccountInput);
    }

    constraints::token_2022::validate_liquidity_token_extensions(
        &accounts.reserve_liquidity_mint.to_account_info(),
        &accounts.user_destination_liquidity.to_account_info(),
    )?;

    Ok(())
}

pub fn repay_obligation_liquidity_checks(accounts: &RepayObligationLiquidity) -> Result<()> {
    let repay_reserve = accounts.repay_reserve.load()?;

    if repay_reserve.liquidity.supply_vault == accounts.user_source_liquidity.key() {
        msg!("Repay reserve liquidity supply cannot be used as the source liquidity provided");
        return err!(LendingError::InvalidAccountInput);
    }

    if repay_reserve.version != PROGRAM_VERSION as u64 {
        msg!("Reserve version does not match the program version");
        return err!(LendingError::ReserveDeprecated);
    }

    constraints::token_2022::validate_liquidity_token_extensions(
        &accounts.reserve_liquidity_mint.to_account_info(),
        &accounts.user_source_liquidity.to_account_info(),
    )?;

    Ok(())
}

pub fn withdraw_obligation_collateral_checks(
    accounts: &WithdrawObligationCollateralAccounts,
) -> Result<()> {
    let withdraw_reserve = accounts.withdraw_reserve.load()?;

    if withdraw_reserve.version != PROGRAM_VERSION as u64 {
        msg!("Reserve version does not match the program version");
        return err!(LendingError::ReserveDeprecated);
    }
    if withdraw_reserve.collateral.supply_vault == accounts.user_destination_collateral.key() {
        msg!("Withdraw reserve collateral supply cannot be used as the destination collateral provided");
        return err!(LendingError::InvalidAccountInput);
    }

    Ok(())
}

pub fn flash_borrow_reserve_liquidity_checks(
    ctx: &Context<FlashBorrowReserveLiquidity>,
) -> Result<()> {
    let reserve = ctx.accounts.reserve.load()?;

    if reserve.liquidity.supply_vault == ctx.accounts.user_destination_liquidity.key() {
        msg!(
            "Borrow reserve liquidity supply cannot be used as the destination liquidity provided"
        );
        return err!(LendingError::InvalidAccountInput);
    }

    if reserve.version != PROGRAM_VERSION as u64 {
        msg!("Reserve version does not match the program version");
        return err!(LendingError::ReserveDeprecated);
    }

    if reserve.config.status() == ReserveStatus::Obsolete {
        msg!("Reserve is obsolete");
        return err!(LendingError::ReserveObsolete);
    }

    if reserve.config.fees.flash_loan_fee_sf == u64::MAX {
        msg!("Flash loans are disabled for this reserve");
        return err!(LendingError::FlashLoansDisabled);
    }

    constraints::token_2022::validate_liquidity_token_extensions(
        &ctx.accounts.reserve_liquidity_mint.to_account_info(),
        &ctx.accounts.user_destination_liquidity.to_account_info(),
    )?;

    Ok(())
}

pub fn flash_repay_reserve_liquidity_checks(
    ctx: &Context<FlashRepayReserveLiquidity>,
) -> Result<()> {
    let reserve = ctx.accounts.reserve.load()?;

    if reserve.liquidity.supply_vault == ctx.accounts.user_source_liquidity.key() {
        msg!("Reserve liquidity supply cannot be used as the source liquidity provided");
        return err!(LendingError::InvalidAccountInput);
    }

    Ok(())
}

pub fn refresh_obligation_farms_for_reserve_checks(
    accounts: &RefreshObligationFarmsForReserveBase,
) -> Result<()> {
    if !accounts.obligation.data_is_empty() {
        let obligation_account: FatAccountLoader<Obligation> =
            FatAccountLoader::try_from(&accounts.obligation).unwrap();
        let obligation = obligation_account.load()?;

        if obligation.lending_market != accounts.lending_market.key() {
            msg!("Obligation lending market does not match the lending market provided");
            return Err(error!(LendingError::InvalidAccountInput)
                .with_pubkeys((obligation.lending_market, accounts.lending_market.key())));
        }
    }

    let reserve = accounts.reserve.load()?;

    if reserve.config.status() == ReserveStatus::Obsolete {
        msg!("Reserve is not active");
        return err!(LendingError::ReserveObsolete);
    }

    if reserve.version != PROGRAM_VERSION as u64 {
        msg!("Reserve version does not match the program version");
        return err!(LendingError::ReserveDeprecated);
    }

    Ok(())
}

pub fn initial_liquidation_reserve_liquidity_available_amount(
    repay_reserve: &AccountLoader<Reserve>,
    withdraw_reserve: &AccountLoader<Reserve>,
) -> (u64, u64) {
    let repay_reserve = repay_reserve.load().unwrap();
    let withdraw_reserve = withdraw_reserve.load().unwrap();
    let repay_reserve_liquidity = repay_reserve.liquidity.available_amount;
    let withdraw_reserve_liquidity = withdraw_reserve.liquidity.available_amount;

    (repay_reserve_liquidity, withdraw_reserve_liquidity)
}

pub fn post_transfer_vault_balance_liquidity_reserve_checks(
    final_reserve_vault_balance: u64,
    final_reserve_available_liquidity: u64,
    initial_reserve_vault_balance: u64,
    initial_reserve_available_liquidity: u64,
    action_type: LendingAction,
) -> anchor_lang::Result<()> {
    let pre_transfer_reserve_diff =
        initial_reserve_vault_balance - initial_reserve_available_liquidity;
    let post_transfer_reserve_diff =
        final_reserve_vault_balance - final_reserve_available_liquidity;

    require_eq!(
        pre_transfer_reserve_diff,
        post_transfer_reserve_diff,
        LendingError::ReserveTokenBalanceMismatch
    );

    match action_type {
        LendingAction::Additive(amount_transferred) => {
            let expected_reserve_vault_balance = initial_reserve_vault_balance + amount_transferred;
            require_eq!(
                expected_reserve_vault_balance,
                final_reserve_vault_balance,
                LendingError::ReserveVaultBalanceMismatch,
            );

            let expected_reserve_available_liquidity =
                initial_reserve_available_liquidity + amount_transferred;
            require_eq!(
                expected_reserve_available_liquidity,
                final_reserve_available_liquidity,
                LendingError::ReserveAccountingMismatch
            );
        }
        LendingAction::Subtractive(amount_transferred) => {
            let expected_reserve_vault_balance = initial_reserve_vault_balance - amount_transferred;
            require_eq!(
                expected_reserve_vault_balance,
                final_reserve_vault_balance,
                LendingError::ReserveVaultBalanceMismatch
            );

            let expected_reserve_available_liquidity =
                initial_reserve_available_liquidity - amount_transferred;
            require_eq!(
                expected_reserve_available_liquidity,
                final_reserve_available_liquidity,
                LendingError::ReserveAccountingMismatch
            );
        }
        LendingAction::SubstractiveSigned(amount_transferred) => {
            let expected_reserve_vault_balance =
                u64::try_from(initial_reserve_vault_balance as i64 - amount_transferred)
                    .map_err(|_| LendingError::MathOverflow)?;
            require_eq!(
                expected_reserve_vault_balance,
                final_reserve_vault_balance,
                LendingError::ReserveVaultBalanceMismatch
            );

            let expected_reserve_available_liquidity =
                u64::try_from(initial_reserve_available_liquidity as i64 - amount_transferred)
                    .map_err(|_| LendingError::MathOverflow)?;
            require_eq!(
                expected_reserve_available_liquidity,
                final_reserve_available_liquidity,
                LendingError::ReserveAccountingMismatch
            );
        }
    }

    Ok(())
}

pub fn post_liquidate_repay_amount_check(max_repay: u64, actual_repay: u64) -> Result<()> {
    require_gte!(
        max_repay,
        actual_repay,
        LendingError::InsufficientRepayAmount
    );
    Ok(())
}

pub fn validate_referrer_token_state(
    program_id: &Pubkey,
    referrer_token_state: &ReferrerTokenState,
    referrer_token_state_key: Pubkey,
    mint: Pubkey,
    owner_referrer: Pubkey,
    reserve_key: Pubkey,
) -> anchor_lang::Result<()> {
    if referrer_token_state.mint == Pubkey::default()
        || referrer_token_state.referrer == Pubkey::default()
    {
        return err!(LendingError::ReferrerAccountNotInitialized);
    }

    if referrer_token_state.mint != mint {
        return err!(LendingError::ReferrerAccountMintMissmatch);
    }

    let referrer_token_state_valid_pda = Pubkey::create_program_address(
        &[
            BASE_SEED_REFERRER_TOKEN_STATE,
            referrer_token_state.referrer.as_ref(),
            reserve_key.as_ref(),
            &[referrer_token_state.bump.try_into().unwrap()],
        ],
        program_id,
    )
    .unwrap();

    if referrer_token_state_key != referrer_token_state_valid_pda {
        return err!(LendingError::ReferrerAccountWrongAddress);
    }

    if referrer_token_state.referrer != owner_referrer {
        return err!(LendingError::ReferrerAccountReferrerMissmatch);
    }

    Ok(())
}
