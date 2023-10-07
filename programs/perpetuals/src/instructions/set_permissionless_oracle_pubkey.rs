//! SetPermissionlessOraclePubkey instruction handler

use {
    crate::state::{
        custody::Custody,
        multisig::{AdminInstruction, Multisig},
        perpetuals::Perpetuals,
        pool::Pool,
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct SetPermissionlessOraclePubkey<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"multisig"],
        bump = multisig.load()?.bump
    )]
    pub multisig: AccountLoader<'info, Multisig>,

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
        mut,
        seeds = [b"custody",
                 pool.key().as_ref(),
                 custody.mint.as_ref()],
        bump = custody.bump
    )]
    pub custody: Box<Account<'info, Custody>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct SetPermissionlessOraclePubkeyParams {
    permissionless_oracle_price_pubkey: [u8; 64],
}

pub fn set_permissionless_oracle_pubkey<'info>(
    ctx: Context<'_, '_, '_, 'info, SetPermissionlessOraclePubkey<'info>>,
    params: &SetPermissionlessOraclePubkeyParams,
) -> Result<u8> {
    let mut multisig = ctx.accounts.multisig.load_mut()?;

    let signatures_left = multisig.sign_multisig(
        &ctx.accounts.admin,
        &Multisig::get_account_infos(&ctx)[1..],
        &Multisig::get_instruction_data(AdminInstruction::SetPermissionlessOraclePubkey, params)?,
    )?;
    if signatures_left > 0 {
        msg!(
            "Instruction has been signed but more signatures are required: {}",
            signatures_left
        );
        return Ok(signatures_left);
    }

    ctx.accounts.custody.permissionless_oracle_price_pubkey =
        Some(params.permissionless_oracle_price_pubkey);
    Ok(0)
}
