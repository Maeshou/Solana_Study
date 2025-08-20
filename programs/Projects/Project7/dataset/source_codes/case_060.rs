use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("EnerA04DripF7lO7Lm3R8tD6W4yZ1nC5bK2hU0S304");

#[program]
pub mod energy_drip_v1 {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, chunk_input: u64, daily_limit_input: u64) -> Result<()> {
        let station = &mut ctx.accounts.station;
        station.manager = ctx.accounts.manager.key();
        station.chunk = chunk_input;
        if station.chunk < 2 { station.chunk = 2; }
        station.daily_limit = daily_limit_input;
        if station.daily_limit < station.chunk { station.daily_limit = station.chunk; }
        station.sent_today = 0;
        station.cool_level = 1;
        Ok(())
    }

    pub fn act_drip(ctx: Context<ActDrip>, bursts: u8) -> Result<()> {
        let station = &mut ctx.accounts.station;

        let mut i: u8 = 0;
        while i < bursts {
            let divisor = station.cool_level + 1;
            let mut amount = station.chunk / divisor;
            if amount < 1 { amount = 1; }

            let projected = station.sent_today + amount;
            if projected > station.daily_limit {
                station.cool_level = station.cool_level + 1;
                return Err(DripErr::DailyLimit.into());
            }

            token::approve(ctx.accounts.approve_ctx(), amount)?;
            token::transfer(ctx.accounts.transfer_ctx(), amount)?;
            token::revoke(ctx.accounts.revoke_ctx())?;

            station.sent_today = projected;

            let milestone = station.chunk * 3;
            if milestone > 0 {
                if station.sent_today % milestone == 0 {
                    if station.cool_level > 0 { station.cool_level = station.cool_level - 1; }
                }
            }
            i = i + 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub station: Account<'info, StationState>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActDrip<'info> {
    #[account(mut, has_one = manager)]
    pub station: Account<'info, StationState>,
    pub manager: Signer<'info>,

    #[account(mut)]
    pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dest_vault: Account<'info, TokenAccount>,
    /// CHECK: 任意の委任先
    pub delegate: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActDrip<'info> {
    pub fn approve_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve {
            to: self.source_vault.to_account_info(),
            delegate: self.delegate.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.source_vault.to_account_info(),
            to: self.dest_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    pub fn revoke_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke {
            source: self.source_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), r)
    }
}
#[account]
pub struct StationState {
    pub manager: Pubkey,
    pub chunk: u64,
    pub daily_limit: u64,
    pub sent_today: u64,
    pub cool_level: u64,
}
#[error_code]
pub enum DripErr { #[msg("daily limit reached")] DailyLimit }
