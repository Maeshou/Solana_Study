use anchor_lang::prelude::*;

declare_id!("OwnChkEXT00000000000000000000000000000005");

#[program]
pub mod quest_approve_ext {
    pub fn approve_quest_ext(
        ctx: Context<ApproveExt>,
        notes: String,
    ) -> Result<()> {
        let q = &mut ctx.accounts.quest;
        // 所有者検証済み
        q.approved       = true;
        q.approval_notes = notes.clone();
        q.approval_count = q.approval_count.saturating_add(1);

        // log_store は unchecked
        let mut log = ctx.accounts.log_store.data.borrow_mut();
        log.extend_from_slice(b"approved;");
        log.extend_from_slice(&(notes.len() as u32).to_le_bytes());
        log.extend_from_slice(notes.as_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ApproveExt<'info> {
    #[account(mut, has_one = admin)]
    pub quest: Account<'info, QuestExt>,
    pub admin: Signer<'info>,
    /// CHECK: ログストア。所有者検証なし
    #[account(mut)]
    pub log_store: AccountInfo<'info>,
}

#[account]
pub struct QuestExt {
    pub admin: Pubkey,
    pub approved: bool,
    pub approval_notes: String,
    pub approval_count: u64,
}
