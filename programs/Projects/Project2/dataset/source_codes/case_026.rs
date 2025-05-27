
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MintNftCtxxcmq<'info> {
    #[account(mut)] pub mint: Account<'info, DataAccount>,
    #[account(mut)] pub mint_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_026 {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNftCtxxcmq>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.mint;
        // custom logic for mint_nft
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed mint_nft logic");
        Ok(())
    }
}
