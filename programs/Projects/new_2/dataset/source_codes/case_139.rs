use anchor_lang::prelude::*;

declare_id!("MixChkBA0000000000000000000000000000000A");

#[program]
pub mod mixed_check20 {
    pub fn penalize(ctx: Context<Pen>, delta: u64) -> Result<()> {
        // rep.moderator と署名者チェックあり
        require_keys_eq!(
            ctx.accounts.rep.moderator,
            ctx.accounts.modr.key(),
            CustomError::Forbidden
        );
        ctx.accounts.rep.score = ctx.accounts.rep.score.saturating_sub(delta);
        // pool_acc は所有者チェックなし
        let _ = ctx.accounts.pool_acc.data.borrow();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Pen<'info> {
    #[account(mut, has_one = moderator)]
    pub rep: Account<'info, Reputation>,
    pub moderator: Signer<'info>,
    #[account(mut)]
    pub pool_acc: AccountInfo<'info>,
}

#[account]
pub struct Reputation {
    pub moderator: Pubkey,
    pub score: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Forbidden")]
    Forbidden,
}
