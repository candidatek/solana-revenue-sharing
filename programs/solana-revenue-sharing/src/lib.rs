pub mod instructions;
pub mod utils;
pub mod state;
pub use state::*;

use anchor_lang::prelude::*;

use instructions::*;
declare_id!("Faw7JrUgvfqjezMFaWhhMC8fVTP41mfUAypeWPP4tGyJ");

#[program]
pub mod solana_revenue_sharing {
    use super::*;

    pub fn initialize(ctx: Context<InitializeGlobalAccounts>) -> Result<()> {
        ctx.accounts.process()?;
        Ok(())
    }
}
