use anchor_lang::prelude::*;

declare_id!("EFQpRZjrbR8QUKzVsrZd7T1dp6fzQDwLZgawjZfxxKEH");

#[program]
pub mod pyth {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
