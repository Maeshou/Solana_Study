use anchor_lang::prelude::*;

// (パターン6のdeclare_id, Guild, GuildErrorを流用)
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_guild_game {
    use super::*;
    pub fn add_member(ctx: Context<AddMember>, new_member_key: Pubkey) -> Result<()> {
        let guild = &mut ctx.accounts.guild_data;
        let max_members_per_level = 10;
        let max_members = guild.level as usize * max_members_per_level;
        
        if guild.members.len() >= max_members {
            return err!(GuildError::GuildFull);
        }
        
        let mut is_already_member = false;
        // forループで既存メンバーかどうかを確認
        for member in guild.members.iter() {
            if *member == new_member_key {
                is_already_member = true;
            }
        }
        
        // is_already_memberがfalseの場合のみ追加
        if !is_already_member {
            guild.members.push(new_member_key);
            msg!("Welcome! {} has joined the guild '{}'.", new_member_key, guild.name);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddMember<'info> {
    #[account(
        mut,
        seeds = [b"guild", guild_data.name.as_bytes()],
        bump = guild_data.bump,
        has_one = leader
    )]
    pub guild_data: Account<'info, Guild>,
    pub leader: Signer<'info>,
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
    NotEnoughMembers,
    GuildFull,
}