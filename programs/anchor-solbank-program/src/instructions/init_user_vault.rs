use crate::states::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
// #[instruction()]
pub struct InitUserVault<'info> {
    #[account(
        init,
        seeds = [initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + 32 + 32 + 8
    )]
    pub user_vault_account: Account<'info, UserVaultAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_user_vault(ctx: Context<InitUserVault>) -> Result<()> {
    let user_vault_account = &mut ctx.accounts.user_vault_account;
    user_vault_account.user_account = ctx.accounts.initializer.key();
    user_vault_account.token_mint = ctx.accounts.initializer.key();
    user_vault_account.token_account = ctx.accounts.initializer.key();
    user_vault_account.balance = 0;
    Ok(())
}
