use anchor_lang::prelude::*;

declare_id!("VulnVarX6000000000000000000000000000000006");

#[program]
pub mod example6 {
    pub fn escalate_privilege(ctx: Context<Ctx6>, level: u8) -> Result<()> {
        // aux_acc は unchecked
        ctx.accounts.aux_acc.lamports.borrow_mut().clone_from(&ctx.accounts.aux_acc.lamports.borrow());

        // privilege は has_one 検証済み
        ctx.accounts.privilege.level = level.max(ctx.accounts.privilege.level);
        ctx.accounts.privilege.upgrade_count = ctx.accounts.privilege.upgrade_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx6<'info> {
    /// CHECK: 補助アカウント、所有者検証なし
    #[account(mut)]
    pub aux_acc: AccountInfo<'info>,

    #[account(mut, has_one = admin)]
    pub privilege: Account<'info, Privilege>,
    pub admin: Signer<'info>,
}

#[account]
pub struct Privilege {
    pub admin: Pubkey,
    pub level: u8,
    pub upgrade_count: u64,
}
