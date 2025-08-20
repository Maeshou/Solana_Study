use anchor_lang::prelude::*;

declare_id!("MixMorA4555555555555555555555555555555555");

#[program]
pub mod mixed_more5 {
    pub fn approve_quest(ctx: Context<Approve>) -> Result<()> {
        let q = &mut ctx.accounts.quest;
        // has_one + Signer で creator を検証
        q.approved = true;
        q.approvals = q.approvals.saturating_add(1);

        // log_cap は未検証
        let mut d = ctx.accounts.log_cap.data.borrow_mut();
        d.fill(0xFF);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(mut, has_one = creator)]
    pub quest: Account<'info, QuestData>,
    pub creator: Signer<'info>,
    /// CHECK: ログキャプチャアカウント
    #[account(mut)]
    pub log_cap: AccountInfo<'info>,
}

#[account]
pub struct QuestData {
    pub creator: Pubkey,
    pub approved: bool,
    pub approvals: u64,
}
