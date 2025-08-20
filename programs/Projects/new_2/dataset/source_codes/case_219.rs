use anchor_lang::prelude::*;

declare_id!("VulnVarXA00000000000000000000000000000000A");

#[program]
pub mod example10 {
    pub fn sync_state(ctx: Context<Ctx10>) -> Result<()> {
        // mirror_acc は unchecked
        let mirror = ctx.accounts.mirror_acc.data.borrow().to_vec();
        // state は has_one 検証済み
        let st = &mut ctx.accounts.state;
        st.data = mirror;
        st.sync_count = st.sync_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx10<'info> {
    /// CHECK: ミラー用アカウント、所有者検証なし
    #[account(mut)]
    pub mirror_acc: AccountInfo<'info>,

    #[account(mut, has_one = owner)]
    pub state: Account<'info, StateData>,
    pub owner: Signer<'info>,
}

#[account]
pub struct StateData {
    pub owner: Pubkey,
    pub data: Vec<u8>,
    pub sync_count: u64,
}
