use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Revoke, Transfer, Token, TokenAccount};

declare_id!("EnergyV4k9Ws2Pl5Rt8Ux1AaBbCcDdEeFfGgHhIiJj007");

#[program]
pub mod energy_distribution_v4 {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, daily_limit: u64, chunk_units: u64) -> Result<()> {
        let station = &mut ctx.accounts.station;
        station.manager_key = ctx.accounts.manager.key();
        station.daily_limit = daily_limit.max(12);
        station.units_sent_today = chunk_units.max(4);
        station.chunk_units = chunk_units.max(4);
        station.stop_flag = false;
        Ok(())
    }

    pub fn act_dispatch(ctx: Context<ActDispatch>, burst_times: u8) -> Result<()> {
        let station = &mut ctx.accounts.station;
        require!(!station.stop_flag, ErrEnergy::Paused);

        let mut burst_cursor = 0u8;
        while burst_cursor < burst_times {
            let projected = station.units_sent_today.saturating_add(station.chunk_units);
            if projected > station.daily_limit {
                station.stop_flag = true;
                return Err(ErrEnergy::OverLimit.into());
            }

            token::approve(ctx.accounts.approve_ctx(), station.chunk_units)?;
            token::transfer(ctx.accounts.transfer_ctx(), station.chunk_units)?;
            token::revoke(ctx.accounts.revoke_ctx())?;

            station.units_sent_today = projected;
            burst_cursor = burst_cursor.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub station: Account<'info, EnergyStation>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActDispatch<'info> {
    #[account(mut, has_one = manager_key)]
    pub station: Account<'info, EnergyStation>,
    pub manager_key: Signer<'info>,

    #[account(mut)]
    pub distributor_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_vault: Account<'info, TokenAccount>,
    /// CHECK: 委任先アドレス
    pub delegate_address: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActDispatch<'info> {
    pub fn approve_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve {
            to: self.distributor_vault.to_account_info(),
            delegate: self.delegate_address.to_account_info(),
            authority: self.manager_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.distributor_vault.to_account_info(),
            to: self.recipient_vault.to_account_info(),
            authority: self.manager_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    pub fn revoke_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke {
            source: self.distributor_vault.to_account_info(),
            authority: self.manager_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), r)
    }
}

#[account]
pub struct EnergyStation {
    pub manager_key: Pubkey,
    pub daily_limit: u64,
    pub units_sent_today: u64,
    pub chunk_units: u64,
    pub stop_flag: bool,
}

#[error_code]
pub enum ErrEnergy {
    #[msg("Dispatch paused")]
    Paused,
    #[msg("Limit exceeded")]
    OverLimit,
}
