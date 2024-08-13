use solana_program::pubkey;

use anchor_lang::prelude::Pubkey;

pub trait PDAIdentifier {
    const IDENT: &'static [u8];

    /// The program that owns this account type
    fn program_id() -> &'static Pubkey;

    /// Get the PDA address (and not the bump). Takes non-const seeds that
    /// come after [Self::IDENT].
    fn get_address(seeds: &[&[u8]]) -> Pubkey {
        Self::get_address_with_bump(seeds).0
    }

    /// Get the PDA address and bump. Takes non-const seeds that
    /// come after [Self::IDENT].
    fn get_address_with_bump(seeds: &[&[u8]]) -> (Pubkey, u8) {
        // TODO: avoid heap allocation
        let mut seeds = seeds.to_vec();
        seeds.insert(0, Self::IDENT);
        Pubkey::find_program_address(&seeds, Self::program_id())
    }
}

pub const USDC_MINT: Pubkey = pubkey!("EaZZs94BAa4KvrNGv5p8uVu8PYV2FxKdtpkB7HpS14ia");
pub const TOKEN_MINT: Pubkey = pubkey!("5ezeNH8Y66UxWu3Q2mc3w3z7qS49ZCmevZqthpUDqSVg");

