use anchor_lang::prelude::*;

declare_id!("OwnChkC8000000000000000000000000000000008");

#[program]
pub mod reputation_mod {
    pub fn penalize(
        ctx: Context<Penalize>,
        amount: u64,
    ) -> Result<()> {
        let r = &mut ctx.accounts.rep;
        // 属性レベルで moderator を検証
        r.score = r.score.saturating_sub(amount);
        r.penalty_count = r.penalty_count.saturating_add(1);

        // audit_cache は unchecked
        ctx.accounts.audit_cache.data.borrow_mut().push(amount as u8);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Penalize<'info> {
    #[account(mut, has_one = moderator)]
    pub rep: Account<'info, RepData>,
    pub moderator: Signer<'info>,
    /// CHECK: 監査キャッシュ、所有者検証なし
    #[account(mut)]
    pub audit_cache: AccountInfo<'info>,
}

#[account]
pub struct RepData {
    pub moderator: Pubkey,
    pub score: u64,
    pub penalty_count: u64,
}
