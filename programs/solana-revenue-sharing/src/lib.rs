pub mod instructions;
use anchor_lang::prelude::*;

use instructions::InitializeGlobalAccounts;
declare_id!("HdWuDYmr65hnPnSs11Z9XQQcJ2Tmqz6y37y9TMdSQeDS");

#[program]
pub mod solana_revenue_sharing {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
