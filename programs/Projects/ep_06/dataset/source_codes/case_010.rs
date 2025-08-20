use anchor_lang::prelude::*;
declare_id!("MINT0101111111111111111111111111111111111111");

#[program]
pub mod case010 {
    use super::*;
    pub fn execute_mintnft(ctx: Context<MintNFTContext>) -> Result<()> {
        // Mint a new NFT
        let mut mint = MintAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        mint.supply = mint.supply.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFTContext<'info> {
    /// CHECK: expecting MintNFTAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting MintNFTAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MintNFTAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}