// 01. ギルド管理：リーダー・監査ログに同一構造体使い回し
use anchor_lang::prelude::*;

declare_id!("G1ldM4n4g3m3nT111111111111111111111111111111");

#[program]
pub mod guild_management {
    use super::*;

    pub fn init_guild(ctx: Context<InitGuild>, name: String) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        guild.name = name;
        guild.leader = ctx.accounts.user.key();
        guild.audit_enabled = false;
        guild.level = 1;
        Ok(())
    }

    pub fn act_toggle_audit_and_level(ctx: Context<ToggleAudit>) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        let log = &mut ctx.accounts.audit_log;

        for i in 0..5 {
            if i % 2 == 0 {
                g.audit_enabled = !g.audit_enabled;
            } else {
                g.level = g.level.wrapping_add(1);
            }
        }

        log.last_action = g.leader;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = user, space = 8 + 32 + 1 + 4 + 32)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ToggleAudit<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub audit_log: Account<'info, Guild>, // Type Cosplay: 同じ構造体を役割違いで利用
    pub checker: AccountInfo<'info>, // Unchecked: 所有者確認なし
}

#[account]
pub struct Guild {
    pub name: String,
    pub leader: Pubkey,
    pub audit_enabled: bool,
    pub level: u32,
    pub last_action: Pubkey,
}
