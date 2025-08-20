use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("GldPnt1010101010101010101010101010101010");

#[program]
pub mod guild_points {
    use super::*;

    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        // ユーザの総貢献を更新
        let total = g.contributions.entry(ctx.accounts.user.key()).or_insert(0);
        *total = total.saturating_add(amount);
        g.overall = g.overall.saturating_add(amount);
        // しきい値超過で昇格フラグを立てる
        if g.overall >= g.promotion_threshold {
            g.promoted = true;
        }
        Ok(())
    }

    pub fn set_threshold(ctx: Context<SetThreshold>, threshold: u64) -> Result<()> {
        ctx.accounts.guild.promotion_threshold = threshold;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub guild: Account<'info, GuildData>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetThreshold<'info> {
    #[account(mut)]
    pub guild: Account<'info, GuildData>,
    pub admin: Signer<'info>,
}

#[account]
pub struct GuildData {
    pub contributions: BTreeMap<Pubkey, u64>,
    pub overall: u64,
    pub promotion_threshold: u64,
    pub promoted: bool,
}
