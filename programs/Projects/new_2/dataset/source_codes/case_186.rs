use anchor_lang::prelude::*;

declare_id!("OwnChkC8000000000000000000000000000000008");

#[program]
pub mod lottery {
    pub fn enter(
        ctx: Context<Enter>,
        ticket: u64,
    ) -> Result<()> {
        let l = &mut ctx.accounts.lottery;
        // 属性検証で l.organizer をチェック
        l.entries.push((ctx.accounts.participant.key(), ticket));
        l.entry_count = l.entry_count.saturating_add(1);

        // prize_log は unchecked
        ctx.accounts.prize_log.data.borrow_mut().extend_from_slice(&ticket.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Enter<'info> {
    #[account(mut, has_one = organizer)]
    pub lottery: Account<'info, LotteryData>,
    pub organizer: Signer<'info>,
    pub participant: Signer<'info>,
    /// CHECK: プライズログ、所有者検証なし
    #[account(mut)]
    pub prize_log: AccountInfo<'info>,
}

#[account]
pub struct LotteryData {
    pub organizer: Pubkey,
    pub entries: Vec<(Pubkey, u64)>,
    pub entry_count: u64,
}
