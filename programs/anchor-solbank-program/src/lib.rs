pub mod instructions;
pub use instructions::*;

pub mod states;
pub use states::*;

use anchor_lang::prelude::*;

declare_id!("DVNJZ33NMmtUsng1muNqzfnNKcCW2EYbKcH3uAxP7tyC");

#[program]
pub mod anchor_solbank_program {
    use super::*;

    pub fn init_user_vault(ctx: Context<InitUserVault>) -> Result<()> {
        instructions::init_user_vault(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }
}
