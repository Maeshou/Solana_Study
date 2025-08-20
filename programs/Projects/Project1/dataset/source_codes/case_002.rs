use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf002mvTWf");

#[program]
pub mod calibrate_access_002 {
    use super::*;

    pub fn calibrate_access(ctx: Context<Ctx002>) -> Result<()> {
        let previous = ctx.accounts.record.access;
        let new_key = ctx.accounts.new_access.key();
        ctx.accounts.record.access = new_key;
        msg!("Case 002: access updated from {} to {}", previous, new_key);
        Ok(())
    }
    pub fn revoke_access(ctx: Context<Ctx002>) -> Result<()> {
        require!(ctx.accounts.manager.is_signer, CustomError::Unauthorized);
        ctx.accounts.record.access = Pubkey::default();
        msg!("Access has been revoked by {}", ctx.accounts.manager.key());
        Ok(())
    }
    pub fn log_access(ctx: Context<Ctx002>) -> Result<()> {
        msg!("Current access: {}", ctx.accounts.record.access);
        Ok(())
    }
    pub fn change_manager(ctx: Context<Ctx002>, new_manager: Pubkey) -> Result<()> {
        require!(ctx.accounts.manager.is_signer, CustomError::Unauthorized);
        ctx.accounts.record.manager = new_manager;
        msg!("Manager changed to {}", new_manager);
        Ok(())
    }


}

#[derive(Accounts)]
pub struct Ctx002<'info> {
    #[account(mut, has_one = manager)]
    pub record: Account<'info, Record002>,
    #[account(signer)]
    pub manager: Signer<'info>,
    pub new_access: Signer<'info>,
}

#[account]
pub struct Record002 {
    pub manager: Pubkey,
    pub access: Pubkey,
}
