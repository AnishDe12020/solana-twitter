use anchor_lang::prelude::*;

declare_id!("A6veVg75tBf31GQMXqdkJWh2goVs99gN7DruUB1Zmsb1");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
