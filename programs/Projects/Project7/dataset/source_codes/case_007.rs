use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Revoke, Transfer, Token, TokenAccount};

declare_id!("En3rgyRegenD1sp4tch1111111111111111111111");

#[program]
pub mod energy_dispatch {
    use super::*;
    pub fn init_station(ctx: Context<InitStation>, daily_cap: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.manager = ctx.accounts.manager.key();
        s.daily_cap = daily_cap;
        s.sent_today = 0;
        s.paused = false;
        Ok(())
    }

    pub fn act_dispatch(ctx: Context<ActDispatch>, units: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;
        require!(!s.paused, ErrorCode::Paused);

        // ループで送付量を丸める（5単位ごと）
        let mut adj = 0;
        for _ in 0..(units / 5) {
            adj += 5;
        }

        let amount = if adj == 0 { 5 } else { adj };
        if s.sent_today.saturating_add(amount) > s.daily_cap {
            s.paused = true;
            return Err(ErrorCode::OverDailyCap.into());
        }

        // approve -> transfer（外部ワークフロー想定）-> revoke
        token::approve(ctx.accounts.approve_ctx(), amount)?;
        token::transfer(ctx.accounts.transfer_ctx(), amount)?;
        token::revoke(ctx.accounts.revoke_ctx())?;

        s.sent_today = s.sent_today.saturating_add(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 1)]
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
    pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_vault: Account<'info, TokenAccount>,
    /// CHECK: Delegate 任意アドレス
    pub delegate: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ActDispatch<'info> {
    pub fn approve_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let accs = Approve {
            to: self.source_vault.to_account_info(),
            delegate: self.delegate.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.source_vault.to_account_info(),
            to: self.player_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
    pub fn revoke_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let accs = Revoke {
            source: self.source_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct EnergyStation {
    pub manager: Pubkey,
    pub daily_cap: u64,
    pub sent_today: u64,
    pub paused: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Station paused")]
    Paused,
    #[msg("Over daily cap")]
    OverDailyCap,
}
