use anchor_lang::prelude::*;
use decimal_wad::decimal::Decimal;


pub const MAX_REWARDS_TOKENS: usize = 10;
pub const REWARD_CURVE_POINTS: usize = 20;


#[derive(Accounts)]
pub struct InitializeFarmDelegated<'info> {
    #[account(mut)]
    pub farm_admin: Signer<'info>,

    #[account(mut)]
    pub farm_delegate: Signer<'info>,

    #[account(zero)]
    pub farm_state: AccountLoader<'info, FarmState>,

    pub global_config: AccountLoader<'info, GlobalConfig>,

    #[account(
        seeds = [BASE_SEED_FARM_VAULTS_AUTHORITY, farm_state.key().as_ref()],
        bump,
    )]
    pub farm_vaults_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}


#[account]
#[derive(Debug, Eq, PartialEq)]
pub struct FarmState {
    pub farm_admin: Pubkey,
    pub global_config: Pubkey,

    pub token: TokenInfo,
    pub reward_infos: [RewardInfo; MAX_REWARDS_TOKENS],
    pub num_reward_tokens: u64,

    pub num_users: u64,
    pub total_staked_amount: u64,

    pub farm_vault: Pubkey,
    pub farm_vaults_authority: Pubkey,
    pub farm_vaults_authority_bump: u64,

    pub delegate_authority: Pubkey,

    pub time_unit: u8,

    pub is_farm_frozen: u8,

    pub is_farm_delegated: u8,

    pub _padding0: [u8; 5],

    pub withdraw_authority: Pubkey,

    pub deposit_warmup_period: u32,
    pub withdrawal_cooldown_period: u32,

    pub total_active_stake_scaled: u128,
    pub total_pending_stake_scaled: u128,

    pub total_pending_amount: u64,

    pub slashed_amount_current: u64,
    pub slashed_amount_cumulative: u64,
    pub slashed_amount_spill_address: Pubkey,

    pub locking_mode: u64,
    pub locking_start_timestamp: u64,
    pub locking_duration: u64,
    pub locking_early_withdrawal_penalty_bps: u64,

    pub deposit_cap_amount: u64,

    pub scope_prices: Pubkey,
    pub scope_oracle_price_id: u64,
    pub scope_oracle_max_age: u64,

    pub pending_farm_admin: Pubkey,
    pub strategy_id: Pubkey,
    pub _padding: [u64; 86],
}

impl Default for FarmState {
    fn default() -> FarmState {
        FarmState {
            farm_admin: Pubkey::default(),
            global_config: Pubkey::default(),

            token: TokenInfo::default(),
            reward_infos: [RewardInfo::default(); MAX_REWARDS_TOKENS],
            num_reward_tokens: 0,

            num_users: 0,
            total_staked_amount: 0,

            farm_vault: Pubkey::default(),
            farm_vaults_authority: Pubkey::default(),
            farm_vaults_authority_bump: 0,

            delegate_authority: Pubkey::default(),
            time_unit: 0,

            is_farm_frozen: 0,
            is_farm_delegated: 0,

            _padding0: [0; 5],

            withdraw_authority: Pubkey::default(),

            deposit_warmup_period: 0,
            withdrawal_cooldown_period: 0,

            total_active_stake_scaled: Decimal::zero().to_scaled_val().unwrap(),
            total_pending_stake_scaled: Decimal::zero().to_scaled_val().unwrap(),
            total_pending_amount: 0,

            slashed_amount_current: 0,
            slashed_amount_cumulative: 0,
            slashed_amount_spill_address: Pubkey::default(),

            locking_mode: 0,
            locking_start_timestamp: 0,
            locking_early_withdrawal_penalty_bps: 0,
            locking_duration: 0,

            deposit_cap_amount: 0,

            scope_prices: Pubkey::default(),
            scope_oracle_price_id: u64::MAX,
            scope_oracle_max_age: u64::MAX,

            pending_farm_admin: Pubkey::default(),
            strategy_id: Pubkey::default(),

            _padding: [0; 86],
        }
    }
}

#[account]
#[derive(Debug, Eq, PartialEq)]
pub struct UserState {
    pub user_id: u64,
    pub farm_state: Pubkey,
    pub owner: Pubkey,

    pub is_farm_delegated: u8,
    pub _padding_0: [u8; 7],

    pub rewards_tally_scaled: [u128; MAX_REWARDS_TOKENS],
    pub rewards_issued_unclaimed: [u64; MAX_REWARDS_TOKENS],
    pub last_claim_ts: [u64; MAX_REWARDS_TOKENS],

    pub active_stake_scaled: u128,

    pub pending_deposit_stake_scaled: u128,
    pub pending_deposit_stake_ts: u64,

    pub pending_withdrawal_unstake_scaled: u128,
    pub pending_withdrawal_unstake_ts: u64,
    pub bump: u64,
    pub delegatee: Pubkey,

    pub last_stake_ts: u64,

    pub _padding_1: [u64; 50],
}

impl Default for UserState {
    fn default() -> UserState {
        UserState {
            user_id: 0,
            farm_state: Pubkey::default(),
            owner: Pubkey::default(),

            is_farm_delegated: false as u8,
            _padding_0: Default::default(),

            rewards_tally_scaled: [0; MAX_REWARDS_TOKENS],
            rewards_issued_unclaimed: [0; MAX_REWARDS_TOKENS],
            last_claim_ts: [0; MAX_REWARDS_TOKENS],

            active_stake_scaled: Decimal::zero().to_scaled_val().unwrap(),
            pending_deposit_stake_scaled: Decimal::zero().to_scaled_val().unwrap(),
            pending_deposit_stake_ts: 0,
            pending_withdrawal_unstake_scaled: Decimal::zero().to_scaled_val().unwrap(),
            pending_withdrawal_unstake_ts: 0,
            bump: 0,
            delegatee: Pubkey::default(),
            last_stake_ts: 0,
            _padding_1: [0; 50],
        }
    }
}


#[account]
#[derive(Debug)]
pub struct GlobalConfig {
    pub global_admin: Pubkey,

    pub treasury_fee_bps: u64,

    pub treasury_vaults_authority: Pubkey,
    pub treasury_vaults_authority_bump: u64,

    pub pending_global_admin: Pubkey,

    pub _padding1: [u128; 126],
}

impl Default for GlobalConfig {
    fn default() -> GlobalConfig {
        GlobalConfig {
            global_admin: Pubkey::default(),

            treasury_vaults_authority: Pubkey::default(),
            treasury_vaults_authority_bump: 0,
            treasury_fee_bps: 0,
            pending_global_admin: Pubkey::default(),
            _padding1: [0; 126],
        }
    }
}


#[account]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct RewardInfo {
    pub token: TokenInfo,

    pub rewards_vault: Pubkey,

    pub rewards_available: u64,
    pub reward_schedule_curve: RewardScheduleCurve,
    pub min_claim_duration_seconds: u64,
    pub last_issuance_ts: u64,
    pub rewards_issued_unclaimed: u64,
    pub rewards_issued_cumulative: u64,
    pub reward_per_share_scaled: u128,
    pub placeholder_0: u64,

    pub reward_type: u8,
    pub rewards_per_second_decimals: u8,

    pub _padding0: [u8; 6],
    pub _padding1: [u64; 20],
}

#[account]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct TokenInfo {
    pub mint: Pubkey,
    pub decimals: u64,
    pub _padding: [u64; 10],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Arbitrary)]
pub struct RewardScheduleCurve {
    pub points: [RewardPerTimeUnitPoint; REWARD_CURVE_POINTS],
}

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Arbitrary)]
#[repr(C)]
pub struct RewardPerTimeUnitPoint {
    pub ts_start: u64,
    pub reward_per_time_unit: u64,
}