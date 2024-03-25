use anchor_lang::prelude::*;

declare_id!("4cd3dyFJyxaostva43V1VVQDR7FUfHZ7uwsgySfr6vg4");

#[program]
pub mod solana_pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
