use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("RaidTeam6PayA7kXn2Vm4Qy6Wt8Rb0Lc3Za5Hd7Q306");

#[program]
pub mod raid_team_distribution_v1 {
    use super::*;

    pub fn init_raid(ctx: Context<InitRaid>, base_chunk_input: u64, daily_limit_input: u64) -> Result<()> {
        let raid = &mut ctx.accounts.raid;
        raid.leader = ctx.accounts.leader.key();
        raid.base_chunk = base_chunk_input;
        if raid.base_chunk < 2 { raid.base_chunk = 2; }
        raid.daily_limit = daily_limit_input;
        if raid.daily_limit < raid.base_chunk { raid.daily_limit = raid.base_chunk; }
        raid.sent_today = 0;
        raid.role_mode = RoleMode::Balanced;
        Ok(())
    }

    pub fn act_distribute(ctx: Context<ActDistribute>, member_count: u8, is_support_heavy: bool) -> Result<()> {
        let raid = &mut ctx.accounts.raid;

        // ベース配布量
        let mut grant_units: u64 = raid.base_chunk;
        let mut member_cursor: u8 = 1;
        while member_cursor < member_count {
            grant_units = grant_units + 1;
            member_cursor = member_cursor + 1;
        }

        // 役割補正
        if is_support_heavy { grant_units = grant_units + 2; }
        if raid.role_mode == RoleMode::Aggressive { grant_units = grant_units + grant_units / 10; }
        if raid.role_mode == RoleMode::Defensive { grant_units = grant_units - grant_units / 12; }

        let projected = raid.sent_today + grant_units;
        if projected > raid.daily_limit {
            return Err(RaidErr::DailyLimit.into());
        }

        token::approve(ctx.accounts.approve_ctx(), grant_units)?;
        token::transfer(ctx.accounts.transfer_ctx(), grant_units)?;
        token::revoke(ctx.accounts.revoke_ctx())?;

        raid.sent_today = projected;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRaid<'info> {
    #[account(init, payer = leader, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub raid: Account<'info, RaidState>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActDistribute<'info> {
    #[account(mut, has_one = leader)]
    pub raid: Account<'info, RaidState>,
    pub leader: Signer<'info>,

    #[account(mut)]
    pub raid_treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub member_vault: Account<'info, TokenAccount>,
    /// CHECK: 任意の委任先
    pub delegate: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActDistribute<'info> {
    pub fn approve_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let call = Approve { to: self.raid_treasury.to_account_info(), delegate: self.delegate.to_account_info(), authority: self.leader.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let call = Transfer { from: self.raid_treasury.to_account_info(), to: self.member_vault.to_account_info(), authority: self.leader.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
    pub fn revoke_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let call = Revoke { source: self.raid_treasury.to_account_info(), authority: self.leader.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
}
#[account]
pub struct RaidState {
    pub leader: Pubkey,
    pub base_chunk: u64,
    pub daily_limit: u64,
    pub sent_today: u64,
    pub role_mode: RoleMode,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RoleMode { Defensive, Balanced, Aggressive }
#[error_code]
pub enum RaidErr { #[msg("daily distribution limit reached")] DailyLimit }
