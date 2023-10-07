//! SetCustomOraclePricePermissionless instruction handler

use {
    crate::{
        error::PerpetualsError,
        state::{custody::Custody, oracle::CustomOracle, perpetuals::Perpetuals, pool::Pool},
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct SetCustomOraclePricePermissionless<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"perpetuals"],
        bump = perpetuals.perpetuals_bump
    )]
    pub perpetuals: Box<Account<'info, Perpetuals>>,

    #[account(
        seeds = [b"pool",
                 pool.name.as_bytes()],
        bump = pool.bump
    )]
    pub pool: Box<Account<'info, Pool>>,

    #[account(
        seeds = [b"custody",
                 pool.key().as_ref(),
                 custody.mint.as_ref()],
        bump = custody.bump
    )]
    pub custody: Box<Account<'info, Custody>>,

    #[account(
        // Custom oracle must first be initialized by authority before permissionless updates.
        mut,
        seeds = [b"oracle_account",
                 pool.key().as_ref(),
                 custody.mint.as_ref()],
        bump
    )]
    pub oracle_account: Box<Account<'info, CustomOracle>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct SetCustomOraclePricePermissionlessParams {
    pub price: u64,
    pub expo: i32,
    pub conf: u64,
    pub ema: u64,
    pub publish_time: i64,
    pub signature: [u8; 64],
    pub recovery_id: u8,
}

pub fn set_custom_oracle_price_permissionless<'info>(
    ctx: Context<'_, '_, '_, 'info, SetCustomOraclePricePermissionless<'info>>,
    params: &SetCustomOraclePricePermissionlessParams,
) -> Result<()> {
    // The new oracle price publish time must be ahead of the current.
    require_gte!(
        params.publish_time,
        ctx.accounts.oracle_account.publish_time,
        PerpetualsError::StaleOraclePrice
    );
    ctx.accounts.oracle_account.set(
        params.price,
        params.expo,
        params.conf,
        params.ema,
        params.publish_time,
    );
    ctx.accounts.oracle_account.verify_signature(
        &ctx.accounts.custody.key(),
        params.signature,
        params.recovery_id,
        ctx.accounts
            .custody
            .permissionless_oracle_price_pubkey
            .ok_or(PerpetualsError::PermissionlessOraclePriceUpdateFailed)?,
    )?;
    Ok(())
}
