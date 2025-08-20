use anchor_lang::prelude::*;

declare_id!("OwnChkC9000000000000000000000000000000009");

#[program]
pub mod leaderboard {
    pub fn reset(
        ctx: Context<ResetBoard>,
    ) -> Result<()> {
        let lb = &mut ctx.accounts.board;
        // 属性検証で lb.admin をチェック
        lb.scores.clear();
        lb.reset_count = lb.reset_count.saturating_add(1);

        // reset_log は unchecked
        ctx.accounts.reset_log.data.borrow_mut().extend_from_slice(&lb.reset_count.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ResetBoard<'info> {
    #[account(mut, has_one = admin)]
    pub board: Account<'info, BoardData>,
    pub admin: Signer<'info>,
    /// CHECK: リセットログ、所有者検証なし
    #[account(mut)]
    pub reset_log: AccountInfo<'info>,
}

#[account]
pub struct BoardData {
    pub admin: Pubkey,
    pub scores: Vec<(Pubkey, u64)>,
    pub reset_count: u64,
}
