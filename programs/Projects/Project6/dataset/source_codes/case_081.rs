use anchor_lang::prelude::*;

declare_id!("22222222222222222222222222222223");

#[program]
pub mod guild_management_program {
    use super::*;

    pub fn create_guild_alliance(
        ctx: Context<CreateGuildAlliance>,
        guild_name: String,
        initial_treasury: u64,
    ) -> Result<()> {
        let guild_account = &mut ctx.accounts.guild_account;
        let founder_account = &mut ctx.accounts.founder_account;
        
        require!(
            guild_name.len() >= 3 && guild_name.len() <= 32,
            GuildError::InvalidGuildName
        );
        
        guild_account.guild_name = guild_name.clone();
        guild_account.guild_leader = ctx.accounts.authority.key();
        guild_account.member_roster = vec![ctx.accounts.authority.key()];
        guild_account.treasury_balance = initial_treasury;
        guild_account.guild_level = 1;
        guild_account.territory_holdings = Vec::new();
        guild_account.alliance_reputation = 0;
        
        founder_account.guild_membership = Some(guild_account.key());
        founder_account.member_rank = GuildRank::Leader;
        founder_account.contribution_points = 100;
        
        emit!(GuildCreated {
            guild_address: guild_account.key(),
            founder: ctx.accounts.authority.key(),
            guild_name,
        });
        
        Ok(())
    }
    
    pub fn execute_territory_conquest(
        ctx: Context<TerritoryConquest>,
        target_territory_id: u32,
        attacking_force_size: u32,
    ) -> Result<()> {
        let attacking_guild = &mut ctx.accounts.attacking_guild;
        let territory_registry = &ctx.accounts.territory_registry;
        
        require!(
            attacking_guild.guild_level >= 5,
            GuildError::InsufficientGuildLevel
        );
        
        let conquest_cost_base = target_territory_id * 1000;
        let force_multiplier = attacking_force_size / 10;
        let total_conquest_cost = conquest_cost_base + (force_multiplier * 500);
        
        require!(
            attacking_guild.treasury_balance >= total_conquest_cost as u64,
            GuildError::InsufficientTreasuryFunds
        );
        
        let territory_defense_rating = target_territory_id % 100;
        let guild_attack_power = attacking_guild.guild_level * 10 + attacking_force_size;
        
        if guild_attack_power > territory_defense_rating {
            attacking_guild.territory_holdings.push(target_territory_id);
            attacking_guild.treasury_balance -= total_conquest_cost as u64;
            attacking_guild.alliance_reputation += 50;
            
            let territory_income_bonus = target_territory_id / 100;
            attacking_guild.guild_level += territory_income_bonus;
            
            emit!(TerritoryConquered {
                guild: attacking_guild.key(),
                territory_id: target_territory_id,
                conquest_cost: total_conquest_cost as u64,
            });
        } else {
            attacking_guild.treasury_balance -= (total_conquest_cost / 2) as u64;
            return Err(GuildError::ConquestFailed.into());
        }
        
        Ok(())
    }
    
    pub fn distribute_guild_rewards(
        ctx: Context<DistributeRewards>,
        reward_pool_amount: u64,
    ) -> Result<()> {
        let guild_account = &ctx.accounts.guild_account;
        let member_account = &mut ctx.accounts.member_account;
        
        require!(
            member_account.guild_membership == Some(guild_account.key()),
            GuildError::NotGuildMember
        );
        
        let member_count = guild_account.member_roster.len() as u64;
        let base_reward = reward_pool_amount / member_count;
        
        let rank_multiplier = match member_account.member_rank {
            GuildRank::Leader => 3.0,
            GuildRank::Officer => 2.5,
            GuildRank::Veteran => 2.0,
            GuildRank::Member => 1.5,
            GuildRank::Recruit => 1.0,
        };
        
        let contribution_bonus = member_account.contribution_points / 100;
        let final_reward = (base_reward as f64 * rank_multiplier) as u64 + contribution_bonus;
        
        member_account.accumulated_rewards += final_reward;
        member_account.last_reward_timestamp = Clock::get()?.unix_timestamp;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateGuildAlliance<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + GuildAccount::INIT_SPACE
    )]
    pub guild_account: Account<'info, GuildAccount>,
    
    #[account(mut)]
    pub founder_account: Account<'info, PlayerMemberAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TerritoryConquest<'info> {
    #[account(
        mut,
        has_one = guild_leader @ GuildError::NotGuildLeader
    )]
    pub attacking_guild: Account<'info, GuildAccount>,
    
    pub territory_registry: Account<'info, TerritoryRegistry>,
    
    #[account(mut)]
    pub guild_leader: Signer<'info>,
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    pub guild_account: Account<'info, GuildAccount>,
    
    #[account(mut)]
    pub member_account: Account<'info, PlayerMemberAccount>,
}

#[account]
#[derive(InitSpace)]
pub struct GuildAccount {
    #[max_len(32)]
    pub guild_name: String,
    pub guild_leader: Pubkey,
    #[max_len(50)]
    pub member_roster: Vec<Pubkey>,
    pub treasury_balance: u64,
    pub guild_level: u32,
    #[max_len(20)]
    pub territory_holdings: Vec<u32>,
    pub alliance_reputation: u64,
}

#[account]
#[derive(InitSpace)]
pub struct PlayerMemberAccount {
    pub guild_membership: Option<Pubkey>,
    pub member_rank: GuildRank,
    pub contribution_points: u64,
    pub accumulated_rewards: u64,
    pub last_reward_timestamp: i64,
}

#[account]
#[derive(InitSpace)]
pub struct TerritoryRegistry {
    #[max_len(100)]
    pub available_territories: Vec<u32>,
    pub conquest_difficulty_modifier: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum GuildRank {
    Leader,
    Officer,
    Veteran,
    Member,
    Recruit,
}

#[event]
pub struct GuildCreated {
    pub guild_address: Pubkey,
    pub founder: Pubkey,
    pub guild_name: String,
}

#[event]
pub struct TerritoryConquered {
    pub guild: Pubkey,
    pub territory_id: u32,
    pub conquest_cost: u64,
}

#[error_code]
pub enum GuildError {
    #[msg("Guild name must be between 3 and 32 characters")]
    InvalidGuildName,
    #[msg("Guild level insufficient for this operation")]
    InsufficientGuildLevel,
    #[msg("Not enough funds in guild treasury")]
    InsufficientTreasuryFunds,
    #[msg("Territory conquest failed")]
    ConquestFailed,
    #[msg("Player is not a member of this guild")]
    NotGuildMember,
    #[msg("Only guild leader can perform this action")]
    NotGuildLeader,
}