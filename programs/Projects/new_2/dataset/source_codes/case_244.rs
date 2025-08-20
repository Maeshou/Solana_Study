use anchor_lang::prelude::*;

declare_id!("VulnEx51000000000000000000000000000000000051");

#[program]
pub mod reward_sender {
    pub fn send_reward(ctx: Context<Ctx1>, amount: u64) -> Result<()> {
        // reward_acc: OWNER CHECK SKIPPED
        **ctx.accounts.reward_acc.lamports.borrow_mut() = 
            ctx.accounts.reward_acc.lamports().saturating_sub(amount);
        **ctx.accounts.recipient.lamports.borrow_mut() = 
            ctx.accounts.recipient.lamports().saturating_add(amount);

        // audit_log: has_one = admin
        let log = &mut ctx.accounts.audit_log;
        log.entries.push((ctx.accounts.admin.key(), amount));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx1<'info> {
    /// CHECK: lamports を自由に操作可能
    #[account(mut)]
    pub reward_acc: AccountInfo<'info>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    #[account(mut, has_one = admin)]
    pub audit_log: Account<'info, AuditLog>,
    pub admin: Signer<'info>,
}

#[account]
pub struct AuditLog {
    pub admin: Pubkey,
    pub entries: Vec<(Pubkey, u64)>,
}
