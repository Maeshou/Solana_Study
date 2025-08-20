use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Revoke, Transfer, Token, TokenAccount};

declare_id!("EnergyV3b5kU9EnergyV3b5kU9EnergyV3b5kU9En9");

#[program]
pub mod energy_airdrop_v3 {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, daily_limit: u64, chunk: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.manager = ctx.accounts.manager.key();
        s.daily_cap = daily_limit.max(12);
        s.sent_today = chunk.max(4);             // 初期配布済み
        s.chunk_size = chunk.max(4);
        s.paused = false;
        Ok(())
    }

    pub fn act_dispatch(ctx: Context<ActDispatch>, bursts: u8) -> Result<()> {
        let s = &mut ctx.accounts.station;
        require!(!s.paused, ErrEnergy::Paused);

        // バースト回数に応じて複数チャンク送付
        let mut i = 0u8;
        while i < bursts {
            let next_total = s.sent_today.saturating_add(s.chunk_size);
            if next_total > s.daily_cap {
                s.paused = true;
                return Err(ErrEnergy::OverCap.into());
            }

            token::approve(ctx.accounts.approve_ctx(), s.chunk_size)?;
            token::transfer(ctx.accounts.transfer_ctx(), s.chunk_size)?;
            token::revoke(ctx.accounts.revoke_ctx())?;

            s.sent_today = next_total;
            i = i.saturating_add(1);
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
    #[account(mut, has_one = manager)]
    pub station: Account<'info, EnergyStation>,
    pub manager: Signer<'info>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    /// CHECK: 委譲先は自由
    pub delegate: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ActDispatch<'info> {
    pub fn approve_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve {
            to: self.source.to_account_info(),
            delegate: self.delegate.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let a = Transfer {
            from: self.source.to_account_info(),
            to: self.recipient.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    pub fn revoke_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let a = Revoke {
            source: self.source.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
}

#[account]
pub struct EnergyStation {
    pub manager: Pubkey,
    pub daily_cap: u64,
    pub sent_today: u64,
    pub chunk_size: u64,
    pub paused: bool,
}

#[error_code]
pub enum ErrEnergy {
    #[msg("Station paused")]
    Paused,
    #[msg("Daily cap exceeded")]
    OverCap,
}
