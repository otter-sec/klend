use anchor_lang::{prelude::*, Accounts};
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    gen_signer_seeds,
    lending_market::{lending_checks, lending_operations},
    state::{LendingMarket, RedeemReserveCollateralAccounts, Reserve},
    utils::{seeds, token_transfer},
};

pub fn process(ctx: Context<RedeemReserveCollateral>, collateral_amount: u64) -> Result<()> {
    lending_checks::redeem_reserve_collateral_checks(&RedeemReserveCollateralAccounts {
        user_source_collateral: ctx.accounts.user_source_collateral.clone(),
        user_destination_liquidity: ctx.accounts.user_destination_liquidity.clone(),
        reserve: ctx.accounts.reserve.clone(),
        reserve_collateral_mint: ctx.accounts.reserve_collateral_mint.clone(),
        reserve_liquidity_supply: ctx.accounts.reserve_liquidity_supply.clone(),
        lending_market: ctx.accounts.lending_market.clone(),
        lending_market_authority: ctx.accounts.lending_market_authority.clone(),
        owner: ctx.accounts.owner.clone(),
        token_program: ctx.accounts.token_program.clone(),
    })?;

    let reserve = &mut ctx.accounts.reserve.load_mut()?;
    let lending_market = &ctx.accounts.lending_market.load()?;
    let clock = &Clock::get()?;

    let lending_market_key = ctx.accounts.lending_market.key();
    let authority_signer_seeds =
        gen_signer_seeds!(lending_market_key.as_ref(), lending_market.bump_seed as u8);

    lending_operations::refresh_reserve_interest(
        reserve,
        clock.slot,
        lending_market.referral_fee_bps,
    )?;
    let withdraw_liquidity_amount =
        lending_operations::redeem_reserve_collateral(reserve, collateral_amount, clock, true)?;

    msg!(
        "pnl: Redeeming reserve collateral {}",
        withdraw_liquidity_amount
    );

    token_transfer::redeem_reserve_collateral_transfer(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.reserve_collateral_mint.to_account_info(),
        ctx.accounts.user_source_collateral.to_account_info(),
        ctx.accounts.owner.to_account_info(),
        ctx.accounts.reserve_liquidity_supply.to_account_info(),
        ctx.accounts.user_destination_liquidity.to_account_info(),
        ctx.accounts.lending_market_authority.clone(),
        authority_signer_seeds,
        collateral_amount,
        withdraw_liquidity_amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct RedeemReserveCollateral<'info> {
    pub owner: Signer<'info>,

    pub lending_market: AccountLoader<'info, LendingMarket>,

    #[account(mut,
        has_one = lending_market
    )]
    pub reserve: AccountLoader<'info, Reserve>,
    #[account(
        seeds = [seeds::LENDING_MARKET_AUTH, lending_market.key().as_ref()],
        bump = lending_market.load()?.bump_seed as u8,
    )]
    pub lending_market_authority: AccountInfo<'info>,

    #[account(mut,
        address = reserve.load()?.collateral.mint_pubkey
    )]
    pub reserve_collateral_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        address = reserve.load()?.liquidity.supply_vault
    )]
    pub reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        token::mint = reserve_collateral_mint
    )]
    pub user_source_collateral: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        token::mint = reserve.load()?.liquidity.mint_pubkey
    )]
    pub user_destination_liquidity: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}
