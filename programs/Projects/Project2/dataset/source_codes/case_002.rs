
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateAuthorityCtxlntk<'info> {
    #[account(mut)] pub vault: Account<'info, DataAccount>,
    #[account(mut)] pub new_authority: Signer<'info>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_002 {
    use super::*;

    pub fn update_authority(ctx: Context<UpdateAuthorityCtxlntk>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.vault;
        // custom logic for update_authority
        assert!(ctx.accounts.vault.data > 0); acct.data -= amount;
        msg!("Executed update_authority logic");
        Ok(())
    }
}
