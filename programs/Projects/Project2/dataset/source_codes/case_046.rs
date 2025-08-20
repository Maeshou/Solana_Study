use anchor_lang::prelude::*;

declare_id!("ToggleState66666666666666666666666666666666");

#[program]
pub mod status_toggle {
    use super::*;

    pub fn toggle(ctx: Context<Toggle>) -> Result<()> {
        let r = &mut ctx.accounts.rec;
        r.active = !r.active;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Toggle<'info> {
    #[account(mut, constraint = rec.admin == admin.key())]
    pub rec: Account<'info, AdminRecord>,
    pub admin: Signer<'info>,
}

#[account]
pub struct AdminRecord {
    pub admin: Pubkey,
    pub active: bool,
}
