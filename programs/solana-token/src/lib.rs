use anchor_lang::prelude::*;

declare_id!("EFJmYstRaTU3HKcA6J5u5GzrckuoyyMEbsD1Csw9jZEw");

#[program]
pub mod solana_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
