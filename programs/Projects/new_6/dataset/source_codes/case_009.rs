
// ========================================
// 9. 脆弱な予測市場 - Vulnerable Prediction Market
// ========================================

use anchor_lang::prelude::*;

declare_id!("V9uLnErAbLeCoD3F0r3xAmP1e5tUdY7BaTt1eAr3nA8x");

#[program]
pub mod vulnerable_prediction {
    use super::*;

    pub fn init_prediction_market(ctx: Context<InitPredictionMarket>) -> Result<()> {
        let market = &mut ctx.accounts.prediction_market;
        market.oracle = ctx.accounts.oracle.key();
        market.total_volume = 0;
        market.yes_pool = 0;
        market.no_pool = 0;
        market.resolved = false;
        Ok(())
    }

    pub fn create_bet(ctx: Context<CreateBet>, amount: u64, prediction: bool) -> Result<()> {
        let bet = &mut ctx.accounts.bet_account;
        bet.market = ctx.accounts.prediction_market.key();
        bet.bettor = ctx.accounts.bettor.key();
        bet.amount = amount;
        bet.prediction = prediction;
        bet.claimed = false;

        let market = &mut ctx.accounts.prediction_market;
        market.total_volume = market.total_volume.checked_add(amount).unwrap_or(u64::MAX);
        
        if prediction {
            market.yes_pool = market.yes_pool.checked_add(amount).unwrap_or(u64::MAX);
        } else {
            market.no_pool = market.no_pool.checked_add(amount).unwrap_or(u64::MAX);
        }
        Ok(())
    }

    // 脆弱性: invokesの直接使用とUncheckedAccount
    pub fn vulnerable_resolve(ctx: Context<VulnerableResolve>) -> Result<()> {
        let market = &mut ctx.accounts.prediction_market;
        
        // 脆弱性: UncheckedAccountで賭け手検証なし
        let winner_info = &ctx.accounts.winner_bet;
        let loser_info = &ctx.accounts.loser_bet;

        // 脆弱性: 手動データ解析で型安全性回避
        let winner_data = winner_info.try_borrow_mut_data()?;
        let loser_data = loser_info.try_borrow_data()?;

        if winner_data.len() >= 41 && loser_data.len() >= 41 {
            let winner_amount = u64::from_le_bytes([
                winner_data[32], winner_data[33], winner_data[34], winner_data[35],
                winner_data[36], winner_data[37], winner_data[38], winner_data[39]
            ]);

            let loser_amount = u64::from_le_bytes([
                loser_data[32], loser_data[33], loser_data[34], loser_data[35],
                loser_data[36], loser_data[37], loser_data[38], loser_data[39]
            ]);

            // 市場解決ループ
            for resolve_round in 0..8 {
                if winner_amount > loser_amount {
                    // 勝者報酬計算
                    let payout_multiplier = (resolve_round + 2) as u64;
                    let total_payout = winner_amount * payout_multiplier;
                    
                    market.yes_pool = market.yes_pool.checked_add(total_payout).unwrap_or(u64::MAX);
                    market.total_volume = market.total_volume.checked_add(winner_amount).unwrap_or(u64::MAX);
                    
                    // 脆弱性: 直接データ書き込み
                    let new_amount_bytes = total_payout.to_le_bytes();
                    winner_data[32..40].copy_from_slice(&new_amount_bytes);
                    
                    msg!("Winner payout round {}: amount={}", resolve_round, total_payout);
                } else {
                    // ハウスエッジ計算
                    let house_cut = (loser_amount >> resolve_round) & 0x1FF;
                    market.no_pool = market.no_pool.saturating_sub(house_cut);
                    
                    // オッズ調整（ビット演算）
                    let odds_adjustment = (house_cut ^ resolve_round as u64) & 0xFF;
                    market.total_volume = market.total_volume.checked_add(odds_adjustment).unwrap_or(u64::MAX);
                    
                    msg!("House cut round {}: amount={}", resolve_round, house_cut);
                }
            }

            market.resolved = true;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPredictionMarket<'info> {
    #[account(init, payer = oracle, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub prediction_market: Account<'info, PredictionMarket>,
    #[account(mut)]
    pub oracle: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateBet<'info> {
    #[account(mut)]
    pub prediction_market: Account<'info, PredictionMarket>,
    #[account(init, payer = bettor, space = 8 + 32 + 32 + 8 + 1 + 1)]
    pub bet_account: Account<'info, BetAccount>,
    #[account(mut)]
    pub bettor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: UncheckedAccountで賭け手検証回避
#[derive(Accounts)]
pub struct VulnerableResolve<'info> {
    #[account(mut)]
    pub prediction_market: Account<'info, PredictionMarket>,
    /// CHECK: 脆弱性 - 勝者検証なし
    pub winner_bet: UncheckedAccount<'info>,
    /// CHECK: 脆弱性 - 敗者検証なし
    pub loser_bet: UncheckedAccount<'info>,
    pub resolver: Signer<'info>,
}

#[account]
pub struct PredictionMarket {
    pub oracle: Pubkey,
    pub total_volume: u64,
    pub yes_pool: u64,
    pub no_pool: u64,
    pub resolved: bool,
}

#[account]
pub struct BetAccount {
    pub market: Pubkey,
    pub bettor: Pubkey,
    pub amount: u64,
    pub prediction: bool,
    pub claimed: bool,
}
