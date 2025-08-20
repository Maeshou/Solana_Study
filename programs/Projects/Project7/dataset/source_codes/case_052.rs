use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Burn, Revoke, Token, TokenAccount, Mint};

declare_id!("Sla6shActA3D7M1Q9L5Z2X8V4N6C0R3T2Y606");

#[program]
pub mod slashing_flow_v1 {
    use super::*;

    pub fn init_monitor(ctx: Context<InitMonitor>, step_units: u64, max_per_tx: u64) -> Result<()> {
        let monitor = &mut ctx.accounts.monitor;
        monitor.moderator = ctx.accounts.moderator.key();
        monitor.step_units = if step_units < 1 { 1 } else { step_units };
        monitor.max_per_tx = if max_per_tx < 2 { 2 } else { max_per_tx };
        monitor.strike_count = 1;
        Ok(())
    }

    pub fn act_slash(ctx: Context<ActSlash>, severity: u8) -> Result<()> {
        let monitor = &mut ctx.accounts.monitor;

        // 段階係数：1,2,3... * step_units
        let mut computed = 0u64;
        let mut s: u8 = 0;
        while s < severity {
            computed = computed + monitor.step_units * (s as u64 + 1);
            s = s + 1;
        }
        if computed > monitor.max_per_tx { computed = monitor.max_per_tx; }
        if computed < 1 { computed = 1; }

        // 委任 → 焼却 → 解除
        token::approve(ctx.accounts.approve_ctx(), computed)?;
        token::burn(ctx.accounts.burn_ctx(), computed)?;
        token::revoke(ctx.accounts.revoke_ctx())?;

        monitor.strike_count = monitor.strike_count + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMonitor<'info> {
    #[account(init, payer = moderator, space = 8 + 32 + 8 + 8 + 8)]
    pub monitor: Account<'info, MonitorState>,
    #[account(mut)]
    pub moderator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActSlash<'info> {
    #[account(mut, has_one = moderator)]
    pub monitor: Account<'info, MonitorState>,
    pub moderator: Signer<'info>,

    pub penalized_mint: Account<'info, Mint>,
    #[account(mut)]
    pub penalized_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActSlash<'info> {
    pub fn approve_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.penalized_vault.to_account_info(), delegate: self.moderator.to_account_info(), authority: self.moderator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let b = Burn { mint: self.penalized_mint.to_account_info(), from: self.penalized_vault.to_account_info(), authority: self.moderator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), b)
    }
    pub fn revoke_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.penalized_vault.to_account_info(), authority: self.moderator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), r)
    }
}

#[account]
pub struct MonitorState {
    pub moderator: Pubkey,
    pub step_units: u64,
    pub max_per_tx: u64,
    pub strike_count: u64,
}
