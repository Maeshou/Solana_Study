use anchor_lang::prelude::*;

declare_id!("ToggleState66666666666666666666666666666666");

#[program]
pub mod status_toggle {
    use super::*;

    pub fn flip_state(ctx: Context<FlipState>) -> Result<()> {
        let rec = &mut ctx.accounts.rec;
        rec.active = !rec.active;
        rec.operator = ctx.accounts.admin.key();
        rec.history.push(rec.active);
        emit!(StateChanged {
            by: ctx.accounts.admin.key(),
            new_state: rec.active
        });
        Ok(())
    }

    pub fn get_snapshot(ctx: Context<View>) -> Result<StateSnapshot> {
        let rec = &ctx.accounts.rec;
        let snap = StateSnapshot {
            current: rec.active,
            last_operator: rec.operator,
            total_toggles: rec.history.len() as u64,
        };
        Ok(snap)
    }
}

#[derive(Accounts)]
pub struct FlipState<'info> {
    #[account(mut, constraint = rec.admin == admin.key())]
    pub rec: Account<'info, AdminRecord>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct View<'info> {
    pub rec: Account<'info, AdminRecord>,
}

#[account]
pub struct AdminRecord {
    pub admin: Pubkey,
    pub active: bool,
    pub operator: Pubkey,
    pub history: Vec<bool>,
}

#[event]
pub struct StateChanged {
    pub by: Pubkey,
    pub new_state: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct StateSnapshot {
    pub current: bool,
    pub last_operator: Pubkey,
    pub total_toggles: u64,
}
