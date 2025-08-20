// 3) market_fee_rebate
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Mark3tFe3Reb4te0000000000000000000000003");

#[program]
pub mod market_fee_rebate {
    use super::*;

    pub fn open(ctx: Context<Open>, cap_bps: u16) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.owner = ctx.accounts.owner.key();
        s.cap_bps = if cap_bps > 3000 { 3000 } else { cap_bps };
        s.trades = 0;
        s.rebated = 0;
        s.labels = 0;
        Ok(())
    }

    pub fn trade_and_rebate(ctx: Context<TradeAndRebate>, price: u64, qty: u64, tag: String) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.owner == ctx.accounts.owner.key(), Errs::Owner);

        // 取引回数と文字列長の処理
        s.trades = s.trades.saturating_add(1);
        if tag.len() > 2 {
            s.labels = s.labels.saturating_add(tag.len() as u32);
        } else {
            let mut fill = 0;
            while fill < 3 {
                s.labels = s.labels.saturating_add(1);
                fill = fill.saturating_add(1);
            }
        }

        let value = price.saturating_mul(qty);
        let mut rebate = value.saturating_mul(s.cap_bps as u64) / 10_000;

        if s.trades % 2 == 0 {
            // 偶数回は段階的に増額してみる
            let mut hops = 0;
            while hops < 4 {
                rebate = rebate.saturating_add(hops as u64);
                hops = hops.saturating_add(1);
            }
        } else {
            // 奇数回は微減と上限確認
            let mut drops = 0;
            while drops < 2 {
                if rebate > 0 {
                    rebate = rebate.saturating_sub(1);
                }
                drops = drops.saturating_add(1);
            }
            if rebate > value {
                rebate = value;
            }
        }

        s.rebated = s.rebated.saturating_add(rebate);

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.fee_vault.key(),
            ctx.accounts.trader_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            rebate,
        )?;
        invoke(&ix, &[
            ctx.accounts.fee_vault.to_account_info(),
            ctx.accounts.trader_ata.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct State {
    pub owner: Pubkey,
    pub cap_bps: u16,
    pub trades: u32,
    pub rebated: u64,
    pub labels: u32,
}

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 4 + 8 + 4)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TradeAndRebate<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub owner: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub fee_vault: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub trader_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}

#[error_code]
pub enum Errs {
    #[msg("owner mismatch")]
    Owner,
}
