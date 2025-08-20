use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("PredMk4tSettleA9hXk2Wm4Qy6Vt8Rb0Lc3Za5Hd7Q304");

#[program]
pub mod prediction_market_settlement_v1 {
    use super::*;

    pub fn init_market(ctx: Context<InitMarket>, fee_bps_input: u16) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.operator = ctx.accounts.operator.key();
        market.fee_bps = clamp_u16(fee_bps_input, 0, 2000);
        market.round_index = 1;
        market.total_settled = 1;
        market.last_outcome_yes = false;
        Ok(())
    }

    pub fn act_settle(ctx: Context<ActSettle>, yes_pool_units: u64, no_pool_units: u64, outcome_yes_flag: bool) -> Result<()> {
        let market = &mut ctx.accounts.market;

        // プール合計
        let combined_pool: u64 = yes_pool_units + no_pool_units;

        // 勝者側・敗者側の判定
        let mut winner_pool: u64 = yes_pool_units;
        let mut loser_pool: u64 = no_pool_units;
        if !outcome_yes_flag {
            winner_pool = no_pool_units;
            loser_pool = yes_pool_units;
        }

        // フィー
        let fee_amount: u64 = (combined_pool as u128 * market.fee_bps as u128 / 10_000u128) as u64;
        let distributable: u64 = combined_pool - fee_amount;

        // 勝者側への分配（比例）
        let mut winner_payout: u64 = distributable;
        if loser_pool > 0 { winner_payout = distributable; } // すべて勝者へ
        if loser_pool == 0 { winner_payout = distributable; } // 引き分け回避

        token::transfer(ctx.accounts.pool_to_winners(), winner_payout)?;
        token::transfer(ctx.accounts.pool_to_fee(), fee_amount)?;

        market.total_settled = market.total_settled + distributable;
        market.round_index = market.round_index + 1;
        market.last_outcome_yes = outcome_yes_flag;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 2 + 8 + 8 + 1)]
    pub market: Account<'info, MarketState>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActSettle<'info> {
    #[account(mut, has_one = operator)]
    pub market: Account<'info, MarketState>,
    pub operator: Signer<'info>,

    #[account(mut)]
    pub outcome_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winners_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActSettle<'info> {
    pub fn pool_to_winners(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let call = Transfer {
            from: self.outcome_pool_vault.to_account_info(),
            to: self.winners_vault.to_account_info(),
            authority: self.operator.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
    pub fn pool_to_fee(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let call = Transfer {
            from: self.outcome_pool_vault.to_account_info(),
            to: self.fee_vault.to_account_info(),
            authority: self.operator.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
}
#[account]
pub struct MarketState {
    pub operator: Pubkey,
    pub fee_bps: u16,
    pub round_index: u64,
    pub total_settled: u64,
    pub last_outcome_yes: bool,
}
fn clamp_u16(v:u16,lo:u16,hi:u16)->u16{let mut o=v; if o<lo{o=lo;} if o>hi{o=hi;} o}
