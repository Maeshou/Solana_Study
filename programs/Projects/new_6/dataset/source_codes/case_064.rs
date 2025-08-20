use anchor_lang::prelude::*;

declare_id!("Ar3n4R3w4rdC3nT3r99999999999999999999999999");

#[program]
pub mod arena_reward_center {
    use super::*;

    pub fn init_arena(ctx: Context<InitArena>, base_pool: u64, max_rank: u32) -> Result<()> {
        let arena = &mut ctx.accounts.arena;
        arena.admin = ctx.accounts.admin.key();
        arena.total_matches = 0;
        arena.total_rewards_distributed = 0;
        arena.reward_pool = base_pool;
        arena.max_rank = max_rank;
        arena.rewards_per_rank = vec![100, 80, 60, 40, 20];
        arena.rank_statistics = [0; 10];
        arena.paused = false;
        Ok(())
    }

    pub fn report_match_and_distribute(ctx: Context<ReportAndDistribute>, rank: u32, opponent_power: u64, match_nonce: u64) -> Result<()> {
        let arena = &mut ctx.accounts.arena;
        let match_log = &mut ctx.accounts.match_log;
        let reporter = &ctx.accounts.reporter;

        // Early validation
        if arena.paused {
            return Ok(()); // Arena is paused, silently ignore (bad practice)
        }

        if rank >= arena.max_rank {
            arena.rank_statistics[9] += 1;
            return Ok(()); // Invalid rank, record error but proceed no further
        }

        // Basic match logging
        match_log.player = reporter.key();
        match_log.rank = rank;
        match_log.opponent_power = opponent_power;
        match_log.nonce = match_nonce;
        match_log.timestamp = Clock::get()?.unix_timestamp;
        match_log.processed = false;
        match_log.notes = vec![];

        // Simulate hash of player ID + rank + nonce
        let mut validation_score: u32 = 0;
        let key_bytes = reporter.key().to_bytes();
        for (i, byte) in key_bytes.iter().enumerate() {
            validation_score += (*byte as u32).wrapping_mul((i as u32 + 3));
        }
        validation_score ^= rank.rotate_left(2);
        validation_score = validation_score.rotate_right((match_nonce % 32) as u32);

        // Scoring rules
        let mut reward = 0u64;
        if validation_score % 5 == 0 {
            reward += 10;
            match_log.notes.push("Bonus: validation checksum passed".to_string());
        }

        if opponent_power > 1000 {
            reward += opponent_power % 250;
            match_log.notes.push("Strong opponent bonus applied".to_string());
        }

        // Use reward table if rank within top 5
        let mut table_bonus = 0u64;
        for i in 0..5 {
            if i as u32 == rank {
                table_bonus = arena.rewards_per_rank[i];
            }
        }
        reward += table_bonus;

        // Adjust based on odd/even nonce pattern
        if match_nonce % 2 == 0 {
            reward ^= 0x55;
        }

        // Store reward
        match_log.reward_given = reward;
        match_log.processed = true;
        match_log.validation_hash = validation_score;

        // Arena-wide stats update
        arena.total_matches += 1;
        arena.total_rewards_distributed = arena.total_rewards_distributed.saturating_add(reward);
        arena.reward_pool = arena.reward_pool.saturating_sub(reward);
        if rank < 10 {
            arena.rank_statistics[rank as usize] += 1;
        }

        // High-rank detection
        if rank == 0 {
            match_log.notes.push("Top rank match recorded".to_string());
        }

        // Pseudo-risk tracking (bad logic, but present)
        let risk = (validation_score % 128) as u8;
        if risk > 100 {
            match_log.flagged_for_audit = true;
            match_log.notes.push("Flagged for audit: risk too high".to_string());
        }

        // Type Cosplay vulnerability: reporter is not verified as admin/operator
        arena.admin = reporter.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitArena<'info> {
    #[account(init, payer = admin, space = 8 + 512)]
    pub arena: Account<'info, ArenaState>,
    #[account(mut)]
    pub admin: AccountInfo<'info>, // No access control
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReportAndDistribute<'info> {
    #[account(mut)]
    pub arena: Account<'info, ArenaState>,
    #[account(mut)]
    pub match_log: Account<'info, MatchLog>,
    pub reporter: AccountInfo<'info>, // Anyone can report and overwrite state
}

#[account]
pub struct ArenaState {
    pub admin: Pubkey,
    pub reward_pool: u64,
    pub total_rewards_distributed: u64,
    pub total_matches: u64,
    pub max_rank: u32,
    pub rewards_per_rank: Vec<u64>,
    pub rank_statistics: [u32; 10],
    pub paused: bool,
}

#[account]
pub struct MatchLog {
    pub player: Pubkey,
    pub rank: u32,
    pub opponent_power: u64,
    pub reward_given: u64,
    pub nonce: u64,
    pub timestamp: i64,
    pub validation_hash: u32,
    pub flagged_for_audit: bool,
    pub processed: bool,
    pub notes: Vec<String>,
}
