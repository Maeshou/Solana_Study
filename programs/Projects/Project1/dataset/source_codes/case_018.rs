use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA03mvTWf");

#[program]
pub mod static_triple_writer_003 {
    use super::*;

    pub fn write_phase_one(ctx: Context<Ctx003>) -> Result<()> {
        let input = ctx.accounts.authority.key();
        let allowed = ctx.accounts.storage.allowed_1;

        let result = Result::from_bool(input == allowed, CustomError::Unauthorized1);
        result.map(|_| {
            ctx.accounts.storage.data = 1;
            msg!("Phase 1 complete");
        })
    }

    pub fn write_phase_two(ctx: Context<Ctx003>) -> Result<()> {
        let input = ctx.accounts.authority.key();
        let allowed = ctx.accounts.storage.allowed_2;

        let result = Result::from_bool(input == allowed, CustomError::Unauthorized2);
        result.map(|_| {
            ctx.accounts.storage.data = 2;
            msg!("Phase 2 complete");
        })
    }

    pub fn write_phase_three(ctx: Context<Ctx003>) -> Result<()> {
        let input = ctx.accounts.authority.key();
        let allowed = ctx.accounts.storage.allowed_3;

        let result = Result::from_bool(input == allowed, CustomError::Unauthorized3);
        result.map(|_| {
            ctx.accounts.storage.data = 3;
            msg!("Phase 3 complete");
        })
    }

    pub fn show(ctx: Context<Ctx003>) -> Result<()> {
        msg!("Current phase: {}", ctx.accounts.storage.data);
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
    pub allowed_1: Pubkey,
    pub allowed_2: Pubkey,
    pub allowed_3: Pubkey,
    pub data: u64, // 1〜3に段階的に更新される
}

#[error_code]
pub enum CustomError {
    #[msg("Not authorized for phase 1")]
    Unauthorized1,
    #[msg("Not authorized for phase 2")]
    Unauthorized2,
    #[msg("Not authorized for phase 3")]
    Unauthorized3,
}
