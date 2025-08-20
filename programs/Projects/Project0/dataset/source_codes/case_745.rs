use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf745mvTWf");

#[program]
pub mod track_session_745 {
    use super::*;

    pub fn track_session(ctx: Context<TrackSession745>) -> Result<()> {
        let sess_bump = *ctx.bumps.get("session").unwrap();
        let clk = Clock::get()?;
        let s = &mut ctx.accounts.session;
        s.bump = sess_bump;
        s.user = ctx.accounts.user.key();
        s.timestamp = clk.unix_timestamp as u64;
        s.slot = clk.slot;
        msg!(
            "Case 745: bump={} user={} ts={} slot={}",
            sess_bump,
            s.user,
            s.timestamp,
            s.slot
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TrackSession745<'info> {
    #[account(init, seeds = [b"session", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 8 + 8)]
    pub session: Account<'info, Session745>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct Session745 {
    pub bump: u8,
    pub user: Pubkey,
    pub timestamp: u64,
    pub slot: u64,
}
