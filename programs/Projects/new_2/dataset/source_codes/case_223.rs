use anchor_lang::prelude::*;

declare_id!("VulnEx21000000000000000000000000000000000021");

#[program]
pub mod profile_manager2 {
    pub fn set_description(ctx: Context<Ctx1>, desc: String) -> Result<()> {
        // audit_log は未検証
        ctx.accounts.audit_log.data.borrow_mut().extend_from_slice(desc.as_bytes());
        // profile は has_one で owner 検証済み
        let p = &mut ctx.accounts.profile;
        p.description = desc;
        p.update_count = p.update_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx1<'info> {
    pub system_program: Program<'info, System>,
    /// CHECK: 監査ログ、所有者検証なし
    #[account(mut)]
    pub audit_log: AccountInfo<'info>,
    pub caller: Signer<'info>,
    #[account(mut, has_one = owner)]
    pub profile: Account<'info, ProfileData>,
}
