

use anchor_lang::{prelude::*, system_program::Transfer};

#[derive(Accounts)]
pub struct InitializeGlobalAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

}

impl<'info> InitializeGlobalAccounts<'info> {
    pub fn process(&mut self) -> Result<()> {
        // token::transfer(
        //     CpiContext::new(
        //         self.token_program.to_account_info()
        //         , Transfer {
        //             from: self.payer_gofx
        //         })
        // )
        Ok(())
    }
}