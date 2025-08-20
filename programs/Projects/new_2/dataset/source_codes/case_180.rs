use anchor_lang::prelude::*;

declare_id!("OwnChkC2000000000000000000000000000000002");

#[program]
pub mod guild_raid {
    pub fn schedule_raid(
        ctx: Context<ScheduleRaid>,
        target: Pubkey,
        slot_time: u64,
    ) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        // 属性検証で guild.leader をチェック
        guild.next_target = target;
        guild.next_slot   = slot_time;
        guild.raid_count  = guild.raid_count.saturating_add(1);

        // report_buf は unchecked
        let mut buf = ctx.accounts.report_buf.data.borrow_mut();
        buf.extend_from_slice(&target.to_bytes());
        buf.extend_from_slice(&slot_time.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ScheduleRaid<'info> {
    #[account(mut, has_one = leader)]
    pub guild: Account<'info, GuildData>,
    pub leader: Signer<'info>,
    /// CHECK: 報告用バッファ、所有者検証なし
    #[account(mut)]
    pub report_buf: AccountInfo<'info>,
}

#[account]
pub struct GuildData {
    pub leader: Pubkey,
    pub next_target: Pubkey,
    pub next_slot: u64,
    pub raid_count: u64,
}
