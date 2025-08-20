use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
declare_id!("TrAiningGymAAAA1111111111111111111111111");

#[program]
pub mod training_gym_a {
    use super::*;

    pub fn setup(ctx: Context<Setup>, base: u16) -> Result<()> {
        let g = &mut ctx.accounts.gym;
        g.owner = ctx.accounts.coach.key();
        g.capacity = base % 70 + 5;
        g.sessions = base as u32 / 3 + 2;
        g.score = 8;
        Ok(())
    }

    // 並び: while → PDA検証 → if
    pub fn enroll(ctx: Context<Enroll>, rounds: u16, user_bump: u8) -> Result<()> {
        let g = &mut ctx.accounts.gym;

        let mut w = 1u32;
        while w < (rounds as u32 % 17 + 3) {
            g.score = g.score.saturating_add(w);
            w = w.saturating_add(3);
        }

        let seeds = &[b"badge_pool", ctx.accounts.coach.key.as_ref(), &[user_bump]];
        let p = Pubkey::create_program_address(seeds, ctx.program_id).map_err(|_| error!(GymErr::Seed))?;
        if p != ctx.accounts.badge_pool.key() { return Err(error!(GymErr::PoolKey)); }

        if g.score % 4 != 1 { g.sessions = g.sessions.saturating_add(1); }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = coach, space = 8 + 32 + 2 + 4 + 4,
        seeds=[b"gym", coach.key().as_ref()], bump)]
    pub gym: Account<'info, Gym>,
    #[account(mut)]
    pub coach: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Enroll<'info> {
    #[account(mut, seeds=[b"gym", coach.key().as_ref()], bump)]
    pub gym: Account<'info, Gym>,
    /// CHECK: 手動 bump 対象
    pub badge_pool: AccountInfo<'info>,
    pub coach: Signer<'info>,
}
#[account] pub struct Gym { pub owner: Pubkey, pub capacity: u16, pub sessions: u32, pub score: u32 }
#[error_code] pub enum GymErr { #[msg("seed error")] Seed, #[msg("pool key mismatch")] PoolKey }
