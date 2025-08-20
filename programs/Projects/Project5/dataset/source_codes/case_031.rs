// ============================================================================
// 3) Obelisk Puzzles — パズル塔（PDAなし; has_one + constraint三連）
// ============================================================================
declare_id!("OBLS333333333333333333333333333333333");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Phase { Entry, Mid, Apex }

#[program]
pub mod obelisk_puzzles {
    use super::*;

    pub fn init_tower(ctx: Context<InitTower>, goal: u32) -> Result<()> {
        let a = &mut ctx.accounts;
        a.tower.caretaker = a.caretaker.key();
        a.tower.goal = goal;
        a.tower.phase = Phase::Entry;
        Ok(())
    }

    pub fn solve(ctx: Context<Solve>, steps: u32) -> Result<()> {
        let a = &mut ctx.accounts;
        for _ in 0..steps {
            a.player.shards = a.player.shards.saturating_add(4);
            a.log.moves = a.log.moves.saturating_add(3);
            a.log.relic = a.log.relic.saturating_add(1);
        }

        if a.player.shards > a.tower.goal as u64 {
            a.tower.phase = Phase::Apex;
            a.log.relic = a.log.relic.saturating_add(5);
            a.player.stars = a.player.stars.saturating_add(2);
            msg!("apex reached; relic+5 stars+2");
        } else {
            a.tower.phase = Phase::Mid;
            a.player.stars = a.player.stars.saturating_add(1);
            a.log.moves = a.log.moves.saturating_add(2);
            msg!("mid run; stars+1 moves+2");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTower<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub tower: Account<'info, Tower>,
    #[account(init, payer=payer, space=8+32+8+1)]
    pub player: Account<'info, Solver>,
    #[account(init, payer=payer, space=8+4+8)]
    pub log: Account<'info, TowerLog>,
    #[account(mut)] pub payer: Signer<'info>,
    pub caretaker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Solve<'info> {
    #[account(mut, has_one=caretaker, constraint = tower.key() != player.key(), error = ObeliskErr::Dup)]
    pub tower: Account<'info, Tower>,
    #[account(mut, constraint = player.key() != log.key(), error = ObeliskErr::Dup)]
    pub player: Account<'info, Solver>,
    #[account(mut, constraint = tower.key() != log.key(), error = ObeliskErr::Dup)]
    pub log: Account<'info, TowerLog>,
    pub caretaker: Signer<'info>,
}

#[account] pub struct Tower { pub caretaker: Pubkey, pub goal: u32, pub phase: Phase }
#[account] pub struct Solver { pub user: Pubkey, pub shards: u64, pub stars: u32 }
#[account] pub struct TowerLog { pub moves: u32, pub relic: u64 }

#[error_code] pub enum ObeliskErr { #[msg("dup")] Dup }
