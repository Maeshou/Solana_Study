// ============================================================================
// 2) Mount Stable — マウント訓練（PDAあり）
//    防止: constraint / has_one / seeds + assert_ne!
// ============================================================================
declare_id!("MNTB23232323232323232323232323232323");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum StallState { Maintenance, Training, Rest }

#[program]
pub mod mount_stable {
    use super::*;

    pub fn init_stable(ctx: Context<InitStable>, capacity: u32) -> Result<()> {
        ctx.accounts.stable.keeper = ctx.accounts.keeper.key();
        ctx.accounts.stable.capacity = capacity;
        ctx.accounts.stable.state = StallState::Training;
        Ok(())
    }

    pub fn train_mount(ctx: Context<TrainMount>, laps: u32) -> Result<()> {
        assert_ne!(ctx.accounts.stable.key(), ctx.accounts.log.key(), "stable/log must differ");

        for _ in 0..laps {
            ctx.accounts.mount.stamina = ctx.accounts.mount.stamina.saturating_add(6);
            ctx.accounts.mount.grade = ctx.accounts.mount.grade.saturating_add(1);
            ctx.accounts.log.sessions = ctx.accounts.log.sessions.saturating_add(2);
        }

        if ctx.accounts.mount.stamina > ctx.accounts.stable.capacity {
            ctx.accounts.stable.state = StallState::Rest;
            ctx.accounts.log.score = ctx.accounts.log.score.saturating_add(15);
            ctx.accounts.mount.grade = ctx.accounts.mount.grade.saturating_add(2);
            msg!("stamina beyond capacity: switch to Rest, record high score");
        } else {
            ctx.accounts.stable.state = StallState::Training;
            ctx.accounts.log.score = ctx.accounts.log.score.saturating_add(4);
            ctx.accounts.mount.stamina = ctx.accounts.mount.stamina.saturating_add(3);
            msg!("continue training: add score & stamina");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStable<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub stable: Account<'info, Stable>,
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub mount: Account<'info, Mount>,
    #[account(init, payer = payer, space = 8 + 4 + 8, seeds = [b"log", keeper.key().as_ref()], bump)]
    pub log: Account<'info, TrainLog>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub keeper: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TrainMount<'info> {
    #[account(mut, has_one = keeper)]
    pub stable: Account<'info, Stable>,
    #[account(mut, constraint = stable.key() != mount.key(), error = StableErr::Dup)]
    pub mount: Account<'info, Mount>,
    #[account(mut, seeds = [b"log", keeper.key().as_ref()], bump)]
    pub log: Account<'info, TrainLog>,
    pub keeper: Signer<'info>,
}

#[account] pub struct Stable { pub keeper: Pubkey, pub capacity: u32, pub state: StallState }
#[account] pub struct Mount { pub rider: Pubkey, pub stamina: u32, pub grade: u8 }
#[account] pub struct TrainLog { pub sessions: u32, pub score: u64 }
#[error_code] pub enum StableErr { #[msg("dup")] Dup }