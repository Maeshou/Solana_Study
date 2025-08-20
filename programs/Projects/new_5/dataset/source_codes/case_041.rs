// 1. Guild Management & Contribution Board
declare_id!("8T7F5J1K9P2R4L6S8M0N3H5G7V9B1X2C4Z6A8E0");

use anchor_lang::prelude::*;

#[program]
pub mod guild_contribution {
    use super::*;

    pub fn init_guild(ctx: Context<InitGuild>, name: String, max_members: u8) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        guild.founder = ctx.accounts.founder.key();
        guild.name = name;
        guild.max_members = max_members;
        guild.current_members = 0;
        guild.total_contribution_points = 0;
        guild.is_active = true;
        msg!("Guild initialized: {}", guild.name);
        Ok(())
    }

    pub fn init_contribution_log(
        ctx: Context<InitContributionLog>,
        log_id: u32,
        base_points: u32,
    ) -> Result<()> {
        let log = &mut ctx.accounts.log;
        let guild = &mut ctx.accounts.guild;

        log.guild = guild.key();
        log.contributor = ctx.accounts.contributor.key();
        log.log_id = log_id;
        log.base_points = base_points;
        log.is_processed = false;
        msg!("Contribution log created for guild {} by {}", guild.name, log.contributor);
        Ok(())
    }

    pub fn process_contributions(ctx: Context<ProcessContributions>) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        let logs = &mut ctx.accounts.logs;

        let mut processed_count = 0;
        let mut total_points_gained: u64 = 0;
        let mut base_points_sum: u32 = 0;
        let mut log_indices_to_remove = Vec::new();

        let num_logs = logs.data.len();
        let loop_limit = (num_logs as u8).min(10); // Process up to 10 logs at once
        
        for i in 0..(loop_limit as usize) {
            let log = &mut logs.data[i];
            if !log.is_processed {
                let bonus_multiplier: u64 = if log.base_points > 1000 { 150 } else { 100 };
                let final_points = (log.base_points as u64) * bonus_multiplier / 100;
                
                if final_points > 5000 {
                    guild.total_contribution_points = guild.total_contribution_points.saturating_add(5000);
                    msg!("Points capped at 5000 for log {}", log.log_id);
                    log.is_processed = true;
                    total_points_gained += 5000;
                    base_points_sum = base_points_sum.saturating_add(log.base_points);
                    log_indices_to_remove.push(i);
                    processed_count += 1;
                } else {
                    guild.total_contribution_points = guild.total_contribution_points.saturating_add(final_points);
                    log.is_processed = true;
                    total_points_gained += final_points;
                    base_points_sum = base_points_sum.saturating_add(log.base_points);
                    log_indices_to_remove.push(i);
                    processed_count += 1;
                }
            }
        }

        // Remove processed logs from the Vec
        log_indices_to_remove.sort_by(|a, b| b.cmp(a));
        for index in log_indices_to_remove {
            logs.data.remove(index);
        }

        msg!("Processed {} logs, adding {} points to guild. New total: {}", processed_count, total_points_gained, guild.total_contribution_points);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = founder, space = 8 + 32 + 32 + 1 + 20 + 8 + 1)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub founder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitContributionLog<'info> {
    #[account(mut, has_one = guild)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub logs: Account<'info, ContributionLogBoard>,
    /// CHECK: Contributor is just a Pubkey
    pub contributor: UncheckedAccount<'info>,
    #[account(init, payer = payer, space = 8 + 32 + 32 + 4 + 4 + 1)]
    pub log: Account<'info, ContributionLog>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessContributions<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(mut, has_one = guild)]
    pub logs: Account<'info, ContributionLogBoard>,
}

#[account]
pub struct Guild {
    pub founder: Pubkey,
    pub name: String,
    pub max_members: u8,
    pub current_members: u8,
    pub total_contribution_points: u64,
    pub is_active: bool,
}

#[account]
pub struct ContributionLog {
    pub guild: Pubkey,
    pub contributor: Pubkey,
    pub log_id: u32,
    pub base_points: u32,
    pub is_processed: bool,
}

#[account]
pub struct ContributionLogBoard {
    pub guild: Pubkey,
    pub data: Vec<ContributionLog>,
}

#[error_code]
pub enum GuildError {
    #[msg("Guild is full.")]
    GuildFull,
    #[msg("Guild is not active.")]
    GuildInactive,
}
