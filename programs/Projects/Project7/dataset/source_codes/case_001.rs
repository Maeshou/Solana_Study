use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount, Mint};

declare_id!("StAk1ngSeTt1eR111111111111111111111111111");

#[program]
pub mod staking_settlement {
    use super::*;
    pub fn init_pool(ctx: Context<InitPool>, fee_bps: u16, min_lock: u64, tier: RewardTier) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.authority = ctx.accounts.authority.key();
        p.fee_bps = fee_bps.min(500);
        p.min_lock = min_lock;
        p.total_settled = 0;
        p.active = true;
        p.tier = tier;
        Ok(())
    }

    pub fn act_settle(ctx: Context<ActSettle>, staked_amount: u64, epochs: u8) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        require!(p.active, ErrorCode::InactivePool);
        require!(staked_amount >= p.min_lock, ErrorCode::BelowMinimum);

        // 係数を tier と epochs から計算（ループを含む）
        let mut multiplier: u64 = match p.tier {
            RewardTier::Bronze => 10,
            RewardTier::Silver => 12,
            RewardTier::Gold => 15,
            RewardTier::Platinum => 18,
        };
        let mut i = 0;
        while i < epochs {
            multiplier = multiplier.saturating_add(1);
            i += 1;
        }

        // 報酬・手数料計算と分岐
        let gross = staked_amount.saturating_mul(multiplier);
        let fee = gross.saturating_mul(p.fee_bps as u64) / 10_000;
        let net = if gross > fee { gross - fee } else { 0 };

        if net == 0 {
            p.active = false;
            return Err(ErrorCode::NoReward.into());
        } else {
            p.total_settled = p.total_settled.saturating_add(net);
        }

        // CPI: transfer (treasury -> staker)
        let cpi_ctx = ctx.accounts.transfer_ctx();
        token::transfer(cpi_ctx, net)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 2 + 8 + 8 + 1 + 1)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActSettle<'info> {
    #[account(mut, has_one = authority)]
    pub pool: Account<'info, Pool>,
    pub authority: Signer<'info>,

    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>, // authority がオーナー
    #[account(mut)]
    pub staker_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ActSettle<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accounts = Transfer {
            from: self.treasury.to_account_info(),
            to: self.staker_vault.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accounts)
    }
}

#[account]
pub struct Pool {
    pub authority: Pubkey,
    pub fee_bps: u16,
    pub min_lock: u64,
    pub total_settled: u64,
    pub active: bool,
    pub tier: RewardTier,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RewardTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Pool is inactive")]
    InactivePool,
    #[msg("Stake below minimum")]
    BelowMinimum,
    #[msg("No reward to settle")]
    NoReward,
}
