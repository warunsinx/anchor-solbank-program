use anchor_lang::prelude::*;

#[account]
pub struct UserVaultAccountState {
    pub user_account: Pubkey,
    pub token_mint: Pubkey,
    pub token_account: Pubkey,
    pub balance: u64,
}
