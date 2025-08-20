use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Revoke, Transfer, Token, TokenAccount};

declare_id!("EngG07Lp5Cv9Ds2Xe1Rw3Ty8Uz4Io6Nm7Qp0G007");

#[program]
pub mod energy_distribution_backoff_v1 {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, daily_limit: u64, chunk_units: u64) -> Result<()> {
        let station_state = &mut ctx.accounts.station_state;
        station_state.manager = ctx.accounts.manager.key();
        station_state.daily_limit = if daily_limit < 12 { 12 } else { daily_limit };
        station_state.distributed_today = chunk_units + 3;
        station_state.base_chunk = if chunk_units < 4 { 4 } else { chunk_units };
        station_state.cool_level = 1;
        Ok(())
    }

    pub fn act_dispatch(ctx: Context<ActDispatch>, burst_times: u8) -> Result<()> {
        let station_state = &mut ctx.accounts.station_state;

        let mut dispatch_index: u8 = 0;
        while dispatch_index < burst_times {
            // クールレベルに伴うバックオフ
            let divisor = station_state.cool_level + 1;
            let mut effective_chunk: u64 = station_state.base_chunk / divisor;
            if effective_chunk < 1 { effective_chunk = 1; }

            let projected_total: u64 = station_state.distributed_today + effective_chunk;
            if projected_total > station_state.daily_limit {
                station_state.cool_level = station_state.cool_level + 1;
                return Err(EnErr::OverDailyLimit.into());
            }

            token::approve(ctx.accounts.approve_ctx(), effective_chunk)?;
            token::transfer(ctx.accounts.transfer_ctx(), effective_chunk)?;
            token::revoke(ctx.accounts.revoke_ctx())?;

            station_state.distributed_today = projected_total;

            // 節目でクール解除
            let milestone: u64 = station_state.base_chunk * 2;
            if milestone > 0 {
                if station_state.distributed_today % milestone == 0 {
                    if station_state.cool_level > 0 {
                        station_state.cool_level = station_state.cool_level - 1;
                    }
                }
            }

            dispatch_index = dispatch_index + 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8)]
    pub station_state: Account<'info, StationState>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActDispatch<'info> {
    #[account(mut, has_one = manager)]
    pub station_state: Account<'info, StationState>,
    pub manager: Signer<'info>,

    #[account(mut)]
    pub distributor_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_vault: Account<'info, TokenAccount>,
    /// CHECK: 委任先の任意アドレス
    pub delegate_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActDispatch<'info> {
    pub fn approve_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let c = Approve {
            to: self.distributor_vault.to_account_info(),
            delegate: self.delegate_account.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let c = Transfer {
            from: self.distributor_vault.to_account_info(),
            to: self.recipient_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
    pub fn revoke_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let c = Revoke {
            source: self.distributor_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
}
#[account]
pub struct StationState {
    pub manager: Pubkey,
    pub daily_limit: u64,
    pub distributed_today: u64,
    pub base_chunk: u64,
    pub cool_level: u64,
}
#[error_code]
pub enum EnErr {
    #[msg("daily limit exceeded")]
    OverDailyLimit,
}
