use anchor_lang::prelude::*;

declare_id!("GUILD555555555555555555555555555555555555555");

#[program]
pub mod guild_management_program {
    use super::*;
    /// プレイヤーがギルドの宝物庫にゴールドと木材を寄付します。
    pub fn contribute_to_guild_treasury(ctx: Context<ContributeToGuild>, gold_amount: u64, wood_amount: u64) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        let member = &mut ctx.accounts.guild_member;
        
        member.gold_balance = member.gold_balance.saturating_sub(gold_amount);
        member.wood_balance = member.wood_balance.saturating_sub(wood_amount);
        
        guild.treasury_gold = guild.treasury_gold.saturating_add(gold_amount);
        guild.treasury_wood = guild.treasury_wood.saturating_add(wood_amount);
        
        let contribution_score = gold_amount.saturating_add(wood_amount * 2);
        member.total_contribution = member.total_contribution.saturating_add(contribution_score);
        
        msg!("Contributed to guild.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ContributeToGuild<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(mut, has_one = guild, has_one = member_authority)]
    pub guild_member: Account<'info, GuildMember>,
    pub member_authority: Signer<'info>,
}

#[account]
pub struct Guild {
    pub treasury_gold: u64,
    pub treasury_wood: u64,
}

#[account]
pub struct GuildMember {
    pub guild: Pubkey,
    pub member_authority: Pubkey,
    pub gold_balance: u64,
    pub wood_balance: u64,
    pub total_contribution: u64,
}