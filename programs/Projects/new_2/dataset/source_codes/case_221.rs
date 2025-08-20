use anchor_lang::prelude::*;

declare_id!("VulnEx12000000000000000000000000000000000012");

#[program]
pub mod example12 {
    pub fn update_user_score(ctx: Context<Ctx12>, delta: i64) -> Result<()> {
        // audit_buf は所有者チェックなし
        ctx.accounts.audit_buf.data.borrow_mut().extend_from_slice(&delta.to_le_bytes());
        // user_score は has_one で user 検証済み
        let us = &mut ctx.accounts.user_score;
        let new = (us.score as i64 + delta).max(0) as u64;
        us.score = new;
        us.update_count = us.update_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx12<'info> {
    /// CHECK: 監査バッファ、所有者検証なし
    #[account(mut)]
    pub audit_buf: AccountInfo<'info>,

    #[account(mut, has_one = user)]
    pub user_score: Account<'info, UserScore>,
    pub user: Signer<'info>,
}

#[account]
pub struct UserScore {
    pub user: Pubkey,
    pub score: u64,
    pub update_count: u64,
}
