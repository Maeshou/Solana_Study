use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("GuildWar0055555555555555555555555555555555");

#[program]
pub mod guild_war {
    use super::*;

    pub fn record_battle(ctx: Context<Record>, guild_a: u64, guild_b: u64, won: bool) -> Result<()> {
        let wars = &mut ctx.accounts.wars;
        let entry = wars.results.entry((guild_a, guild_b)).or_insert((0, 0));
        if won {
            entry.0 = entry.0.saturating_add(1);
        } else {
            entry.1 = entry.1.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Record<'info> {
    #[account(mut)]
    pub wars: Account<'info, WarResults>,
}

#[account]
pub struct WarResults {
    /// key=(attacker, defender) -> (wins, losses)
    pub results: BTreeMap<(u64, u64), (u64, u64)>,
}
