use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA01mvTWf");

#[program]
pub mod match_guarded_counter_003 {
    use super::*;

    pub fn controlled_increment(ctx: Context<Ctx003>, limit: u64) -> Result<()> {
        // 署名者チェック without if
        match ctx.accounts.authority.is_signer {
            true => (),
            false => return Err(CustomError::Unauthorized.into()),
        }

        // カウント制限チェック without if
        match ctx.accounts.storage.data.checked_add(1) {
            Some(new_val) if new_val <= limit => {
                ctx.accounts.storage.data = new_val;
                msg!("Incremented safely: {}", new_val);
                Ok(())
            },
            _ => Err(CustomError::LimitReached.into()),
        }
    }

    pub fn show(ctx: Context<Ctx003>) -> Result<()> {
        msg!("Count: {}", ctx.accounts.storage.data);
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
    pub data: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Limit reached or overflow")]
    LimitReached,
}
