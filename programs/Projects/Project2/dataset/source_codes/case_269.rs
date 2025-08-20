use anchor_lang::prelude::*;

declare_id!("Stamina088888888888888888888888888888888");

#[program]
pub mod stamina_tracker {
    use super::*;

    pub fn record_action(ctx: Context<ModifyStamina>) -> Result<()> {
        let st = &mut ctx.accounts.stamina;
        st.actions = st.actions.saturating_add(1);
        if st.actions % st.recover_rate == 0 {
            st.recoveries = st.recoveries.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyStamina<'info> {
    #[account(mut)]
    pub stamina: Account<'info, StaminaData>,
}

#[account]
pub struct StaminaData {
    pub actions: u64,
    pub recover_rate: u64,
    pub recoveries: u64,
}
