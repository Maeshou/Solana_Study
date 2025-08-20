use anchor_lang::prelude::*;

declare_id!("VulnEx13000000000000000000000000000000000013");

#[program]
pub mod example13 {
    pub fn ban_user(ctx: Context<Ctx13>, offender: Pubkey) -> Result<()> {
        // audit_log は所有者チェックなし
        let mut buf = ctx.accounts.audit_log.data.borrow_mut();
        buf.extend_from_slice(b"ban:");
        buf.extend_from_slice(&offender.to_bytes());
        // ban_list は has_one で moderator 検証済み
        let bl = &mut ctx.accounts.ban_list;
        bl.banned.push(offender);
        bl.ban_count = bl.ban_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx13<'info> {
    /// CHECK: 監査ログ、所有者検証なし
    #[account(mut)]
    pub audit_log: AccountInfo<'info>,

    #[account(mut, has_one = moderator)]
    pub ban_list: Account<'info, BanList>,
    pub moderator: Signer<'info>,
}

#[account]
pub struct BanList {
    pub moderator: Pubkey,
    pub banned: Vec<Pubkey>,
    pub ban_count: u64,
}
