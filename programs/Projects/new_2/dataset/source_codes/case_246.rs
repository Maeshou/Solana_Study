use anchor_lang::prelude::*;

declare_id!("VulnEx53000000000000000000000000000000000053");

#[program]
pub mod random_draw {
    pub fn draw(ctx: Context<Ctx3>) -> Result<()> {
        // result_buf: OWNER CHECK SKIPPED
        let seed = Clock::get()?.unix_timestamp as u64;
        ctx.accounts.result_buf.data.borrow_mut()[..8]
            .copy_from_slice(&seed.to_le_bytes());

        // draw_state: has_one = manager
        let st = &mut ctx.accounts.draw_state;
        st.last_seed = seed;
        st.counter = st.counter.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx3<'info> {
    pub clock: Sysvar<'info, Clock>,
    #[account(mut)]
    pub result_buf: AccountInfo<'info>,

    #[account(mut, has_one = manager)]
    pub draw_state: Account<'info, DrawState>,
    pub manager: Signer<'info>,
}

#[account]
pub struct DrawState {
    pub manager: Pubkey,
    pub last_seed: u64,
    pub counter: u64,
}
