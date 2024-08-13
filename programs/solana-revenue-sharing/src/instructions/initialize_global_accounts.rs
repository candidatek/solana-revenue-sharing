

use anchor_lang::{prelude::*};
use anchor_spl::token;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount, Transfer}};
use crate::{utils::{TOKEN_MINT, USDC_MINT}, StakePool};
use crate::utils::PDAIdentifier;
#[derive(Accounts)]
pub struct InitializeGlobalAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = payer,
    )]
    pub payer_token: Box<Account<'info, TokenAccount>>,

    #[account(
        address = TOKEN_MINT
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    /// CHECK: Just a PDA Signer
    
    #[account(
        seeds = [
            crate::state::TokenVault::IDENT,
        ],
        bump,
    )]
    pub token_vault_signer: UncheckedAccount<'info>,
    /// CHECK: Just a PDA Signer
    #[account(
        seeds = [
            crate::state::TokenUnstaked::IDENT
        ],
        bump
    )]
    pub token_unstaked_signer: UncheckedAccount<'info>,

    ///CHECK: Just a PDA Signer
    #[account(
        seeds = [crate::state::FeesCollected::IDENT],
        bump,
    )]
    pub usdc_fee_signer: UncheckedAccount<'info>,

    /// CHECK: Just a PDA Signer
    #[account(
        seeds = [
            crate::state::UsdcRewardVault::IDENT
        ],
        bump
    )]
    pub usdc_reward_signer: UncheckedAccount<'info>,
    /// This instruction initializes the token account
    /// required for storing staked token 
    #[account(
        init, 
        payer= payer,
        associated_token::mint = token_mint,
        associated_token::authority = token_vault_signer,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer=payer,
        associated_token::mint = token_mint,
        associated_token::authority = token_unstaked_signer
    )]
    pub token_unstaked_vault: Box<Account<'info, TokenAccount>>,

    /// This instruction initializes the token account
    /// required for storing USDC rewards
    
    #[account(
        init,
        payer = payer,
        associated_token::mint = usdc_mint,
        associated_token::authority = usdc_reward_signer
    )]
    pub usdc_reward_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        address = USDC_MINT
    )]
    pub usdc_mint: Box<Account<'info, TokenAccount>>,
    ///This instruction initializes the global account struct that stores 
    /// the staking information
    
    #[account(
        init,
        space = 8 + std::mem::size_of::<StakePool>(),
        payer = payer,
        seeds = [
            crate::state::StakePool::IDENT
        ],
        bump,
    )]
    pub stake_pool: Box<Account<'info, StakePool>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>

}

impl<'info> InitializeGlobalAccounts<'info> {
    pub fn process(&mut self) -> Result<()> {
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info()
                , Transfer {
                    from: self.payer_token.to_account_info(),
                    to: self.token_vault.to_account_info(),
                    authority: self.payer.to_account_info(),
                },
            ),
            1,
        )?;
        Ok(())
    }
}