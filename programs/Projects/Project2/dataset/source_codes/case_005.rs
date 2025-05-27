
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MintTokenCtxcbtq<'info> {
    #[account(mut)] pub mint: Account<'info, DataAccount>,
    #[account(mut)] pub mint_authority: Signer<'info>,
    #[account(mut)] pub token_program: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_005 {
    use super::*;

    pub fn mint_token(ctx: Context<MintTokenCtxcbtq>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.mint;
        // custom logic for mint_token
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed mint_token logic");
        Ok(())
    }
}
