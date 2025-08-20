use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA04mvTWf");

#[program]
pub mod dual_static_setter_003 {
    use super::*;

    pub fn set_to_777(ctx: Context<Ctx003>) -> Result<()> {
        let key = ctx.accounts.authority.key();
        let allowed = ctx.accounts.storage.allowed_777;
        Result::from_bool(key == allowed, CustomError::NotAllowed777)
            .map(|_| {
                ctx.accounts.storage.data = 777;
                msg!("Data set to 777");
            })
    }

    pub fn set_to_888(ctx: Context<Ctx003>) -> Result<()> {
        let key = ctx.accounts.authority.key();
        let allowed = ctx.accounts.storage.allowed_888;
        Result::from_bool(key == allowed, CustomError::NotAllowed888)
            .map(|_| {
                ctx.accounts.storage.data = 888;
                msg!("Data set to 888");
            })
    }

    pub fn read(ctx: Context<Ctx003>) -> Result<()> {
        msg!("Stored value: {}", ctx.accounts.storage.data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub allowed_777: Pubkey,
    pub allowed_888: Pubkey,
    pub data: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Not authorized to set 777")]
    NotAllowed777,
    #[msg("Not authorized to set 888")]
    NotAllowed888,
}
