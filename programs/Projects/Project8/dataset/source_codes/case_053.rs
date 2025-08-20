use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_guild_game {
    use super::*;

    pub fn create_guild(ctx: Context<CreateGuild>, guild_name: String, initial_members: Vec<Pubkey>) -> Result<()> {
        let guild = &mut ctx.accounts.guild_data;

        let min_members = 3;
        if initial_members.len() < min_members {
            return err!(GuildError::NotEnoughMembers);
        }

        guild.name = guild_name;
        guild.leader = *ctx.accounts.leader.key;
        guild.members = initial_members;
        guild.level = 1;
        guild.bump = *ctx.bumps.get("guild_data").unwrap();

        // ギルドメンバー登録のダミーループ
        for member_key in guild.members.iter() {
            msg!("Registering member: {}", member_key);
        }

        msg!("Guild '{}' has been successfully created!", guild.name);
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(guild_name: String)]
pub struct CreateGuild<'info> {
    #[account(
        init,
        payer = leader,
        space = 8 + 4 + 20 + 32 + 4 + (32 * 10) + 2 + 1, // Max 10 members
        seeds = [b"guild", guild_name.as_bytes()],
        bump
    )]
    pub guild_data: Account<'info, Guild>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Guild {
    pub name: String,
    pub leader: Pubkey,
    pub members: Vec<Pubkey>,
    pub level: u16,
    pub bump: u8,
}

#[error_code]
pub enum GuildError {
    #[msg("Not enough members to create a guild.")]
    NotEnoughMembers,
    #[msg("Guild is already full.")]
    GuildFull,
}