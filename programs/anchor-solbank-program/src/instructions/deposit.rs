use crate::states::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
// #[instruction()]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user_vault_account: Account<'info, UserVaultAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let user_vault_account = &mut ctx.accounts.user_vault_account;
    user_vault_account.balance += amount;
    Ok(())
}
