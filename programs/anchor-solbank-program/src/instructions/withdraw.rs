use crate::states::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
// #[instruction()]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user_vault_account: Account<'info, UserVaultAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user_vault_account = &mut ctx.accounts.user_vault_account;
    user_vault_account.balance -= amount;
    Ok(())
}
