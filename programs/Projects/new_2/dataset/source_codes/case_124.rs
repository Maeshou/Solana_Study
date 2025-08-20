use anchor_lang::prelude::*;

declare_id!("MixChk5555555555555555555555555555555555");

#[program]
pub mod mixed_check5 {
    pub fn conclude(ctx: Context<Conclude>) -> Result<()> {
        // auction.manager は検証あり
        require_keys_eq!(ctx.accounts.auction.manager, ctx.accounts.manager.key(), CustomError::NotManager);
        ctx.accounts.auction.closed = true;
        // event_log_acc は owner check が抜けている
        let _ = ctx.accounts.event_log_acc.data.borrow();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Conclude<'info> {
    #[account(mut, has_one = manager)]
    pub auction: Account<'info, Auction>,
    pub manager: Signer<'info>,

    /// CHECK: ログアカウント未検証
    #[account(mut)]
    pub event_log_acc: AccountInfo<'info>,
}

#[account]
pub struct Auction {
    pub manager: Pubkey,
    pub closed: bool,
}
