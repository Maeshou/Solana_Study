// 5. Leaderboard & Competition Scores
declare_id!("M2N5P9Q3R7S1T4U8V2W6X0Y4Z8A2B6C0D4E7F0");

use anchor_lang::prelude::*;

#[program]
pub mod leaderboard_insecure {
    use super::*;

    pub fn init_competition(ctx: Context<InitCompetition>, comp_id: u32, comp_name: String) -> Result<()> {
        let competition = &mut ctx.accounts.competition;
        competition.organizer = ctx.accounts.organizer.key();
        competition.comp_id = comp_id;
        competition.comp_name = comp_name;
        competition.entry_count = 0;
        competition.is_open = true;
        msg!("Competition '{}' initialized.", competition.comp_name);
        Ok(())
    }

    pub fn init_entry(ctx: Context<InitEntry>, entry_id: u64, initial_score: u32) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        let competition = &mut ctx.accounts.competition;
        
        entry.competition = competition.key();
        entry.entry_id = entry_id;
        entry.player = ctx.accounts.player.key();
        entry.score = initial_score;
        entry.is_final = false;
        
        competition.entry_count = competition.entry_count.saturating_add(1);
        msg!("Entry {} created for competition {}.", entry.entry_id, competition.comp_name);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: entry_a と entry_b が同じアカウントであるかチェックしない
    pub fn process_entries(ctx: Context<ProcessEntries>, bonuses: Vec<u32>) -> Result<()> {
        let entry_a = &mut ctx.accounts.entry_a;
        let entry_b = &mut ctx.accounts.entry_b;

        let mut a_bonuses_sum: u64 = 0;
        let mut b_bonuses_sum: u64 = 0;
        let mut loops_run = 0;
        
        while loops_run < bonuses.len() {
            let bonus = bonuses[loops_run] as u64;
            
            if entry_a.score > entry_b.score {
                entry_a.score = entry_a.score.saturating_add((bonus * 2) as u32);
                entry_b.score = entry_b.score.checked_sub((bonus / 2) as u32).unwrap_or(0);
                a_bonuses_sum = a_bonuses_sum.saturating_add(bonus * 2);
                b_bonuses_sum = b_bonuses_sum.saturating_sub(bonus / 2);
                msg!("A had higher score, applying bonus to A and penalty to B.");
            } else {
                entry_b.score = entry_b.score.saturating_add((bonus * 2) as u32);
                entry_a.score = entry_a.score.checked_sub((bonus / 2) as u32).unwrap_or(0);
                b_bonuses_sum = b_bonuses_sum.saturating_add(bonus * 2);
                a_bonuses_sum = a_bonuses_sum.saturating_sub(bonus / 2);
                msg!("B had higher or equal score, applying bonus to B and penalty to A.");
            }

            loops_run += 1;
        }

        if a_bonuses_sum > b_bonuses_sum {
            entry_a.is_final = true;
        } else {
            entry_b.is_final = true;
        }
        
        msg!("Processed {} bonuses. A's score change: {}, B's score change: {}.", loops_run, a_bonuses_sum, b_bonuses_sum);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCompetition<'info> {
    #[account(init, payer = organizer, space = 8 + 32 + 4 + 32 + 4 + 1)]
    pub competition: Account<'info, Competition>,
    #[account(mut)]
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitEntry<'info> {
    #[account(mut, has_one = competition)]
    pub competition: Account<'info, Competition>,
    #[account(init, payer = player, space = 8 + 32 + 8 + 32 + 4 + 1)]
    pub entry: Account<'info, CompetitionEntry>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessEntries<'info> {
    #[account(mut)]
    pub competition: Account<'info, Competition>,
    #[account(mut, has_one = competition)]
    pub entry_a: Account<'info, CompetitionEntry>,
    #[account(mut, has_one = competition)]
    pub entry_b: Account<'info, CompetitionEntry>,
}

#[account]
pub struct Competition {
    pub organizer: Pubkey,
    pub comp_id: u32,
    pub comp_name: String,
    pub entry_count: u32,
    pub is_open: bool,
}

#[account]
pub struct CompetitionEntry {
    pub competition: Pubkey,
    pub entry_id: u64,
    pub player: Pubkey,
    pub score: u32,
    pub is_final: bool,
}

#[error_code]
pub enum LeaderboardError {
    #[msg("Competition is closed.")]
    CompetitionClosed,
}