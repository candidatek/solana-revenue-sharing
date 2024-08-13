use crate::utils::{PDAIdentifier, TOKEN_MINT, USDC_MINT};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address;

/// A global account that stores global staking information.
#[account]
#[derive(Debug, Copy, Default)]
#[repr(C)]
pub struct StakePool {
    /// A monotonically increasing value, that increments each time
    /// USDC is added to the [UsdcRewardVault]'s associated USDC account.
    pub total_accumulated_profit: u64,
    pub protocol_activated_at: i64,
}

const _: [u8; 16] = [0u8; std::mem::size_of::<StakePool>()];

impl PDAIdentifier for StakePool {
    const IDENT: &'static [u8] = b"stake_pool";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

impl StakePool {
    pub fn address() -> Pubkey {
        Self::get_address(&[])
    }
}
#[account]
#[derive(Debug)]
pub struct TokenVault {}

impl PDAIdentifier for TokenVault {
    const IDENT: &'static [u8] = b"token_vault";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

impl TokenVault {
    pub fn vault_address() -> Pubkey {
        get_associated_token_address(&Self::signer_address(), &TOKEN_MINT)
    }
    pub fn signer_address() -> Pubkey {
        Self::get_address(&[])
    }
}

#[account]
#[derive(Debug)]
pub struct TokenUnstaked {}

impl PDAIdentifier for TokenUnstaked {
    const IDENT: &'static [u8] = b"token_unstaked";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

impl TokenUnstaked {
    pub fn vault_address() -> Pubkey {
        get_associated_token_address(&Self::signer_address(), &TOKEN_MINT)
    }
    pub fn signer_address() -> Pubkey {
        Self::get_address(&[])
    }
}

#[account]
#[derive(Debug)]

pub struct FeesCollected {}

impl PDAIdentifier for FeesCollected {
    const IDENT: &'static [u8] = b"fees_collected";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

impl FeesCollected {
    pub fn usdc_fee_vault() -> Pubkey {
        get_associated_token_address(&Self::signer_address(), &USDC_MINT)
    }

    pub fn fee_vault(mint: &Pubkey) -> Pubkey {
        get_associated_token_address(&Self::signer_address(), mint)
    }

    pub fn signer_address() -> Pubkey {
        Self::get_address(&[])
    }
}

#[account]
#[derive(Debug)]
pub struct UsdcRewardVault {}

impl PDAIdentifier for UsdcRewardVault {
    const IDENT: &'static [u8] = b"usdc_reward_vault";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

impl UsdcRewardVault {
    pub fn vault_address() -> Pubkey {
        get_associated_token_address(
            &Self::signer_address(),
            &USDC_MINT,
        )
    }

    pub fn signer_address() -> Pubkey {
        Self::get_address(&[])
    }
}
