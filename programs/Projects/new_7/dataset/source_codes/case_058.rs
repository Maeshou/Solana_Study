// 3) market_settlement: ミニマーケットの売上精算（複数回の小分け送付）
use anchor_lang::prelude::*;
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("M4rketSettle33333333333333333333333333333");

#[program]
pub mod market_settlement {
    use super::*;
    pub fn open(ctx: Context<Open>, fee_bps: u16) -> Result<()> {
        let m = &mut ctx.accounts.market;
        m.owner = ctx.accounts.owner.key();
        m.fee_bps = fee_bps.min(1200);
        m.turnover = 0;
        m.orders = 0;
        Ok(())
    }

    pub fn settle(ctx: Context<Settle>, gross: u64, parts: u8) -> Result<()> {
        let m = &mut ctx.accounts.market;

        // 手数料と最初の分配
        let fee = gross.saturating_mul(m.fee_bps as u64) / 10_000;
        let mut remain = if gross > fee { gross - fee } else { 0 };

        if remain == 0 {
            // 受注がない時の推移
            m.orders = m.orders.saturating_add(1);
            return Ok(());
        }

        // 小分け送付（while で parts 回）
        let mut i = 0;
        while i < parts {
            let chunk = remain / 2; // 単純な半分ずつ
            if chunk == 0 {
                break;
            }
            let ix = token_ix::transfer(
                &ctx.accounts.any_program.key(),
                &ctx.accounts.treasury.key(),
                &ctx.accounts.seller_vault.key(),
                &ctx.accounts.owner.key(),
                &[],
                chunk,
            )?;
            invoke(
                &ix,
                &[
                    ctx.accounts.any_program.to_account_info(),
                    ctx.accounts.treasury.to_account_info(),
                    ctx.accounts.seller_vault.to_account_info(),
                    ctx.accounts.owner.to_account_info(),
                ],
            )?;
            remain = remain.saturating_sub(chunk);
            i += 1;
        }

        // 最後の残りも送付
        if remain > 0 {
            let ix2 = token_ix::transfer(
                &ctx.accounts.any_program.key(),
                &ctx.accounts.treasury.key(),
                &ctx.accounts.seller_vault.key(),
                &ctx.accounts.owner.key(),
                &[],
                remain,
            )?;
            invoke(
                &ix2,
                &[
                    ctx.accounts.any_program.to_account_info(),
                    ctx.accounts.treasury.to_account_info(),
                    ctx.accounts.seller_vault.to_account_info(),
                    ctx.accounts.owner.to_account_info(),
                ],
            )?;
        }

        m.turnover = m.turnover.saturating_add(gross);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 8 + 8)]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(mut, has_one = owner)]
    pub market: Account<'info, Market>,
    pub owner: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub seller_vault: UncheckedAccount<'info>,
    /// CHECK:
    pub any_program: UncheckedAccount<'info>,
}

#[account]
pub struct Market {
    pub owner: Pubkey,
    pub fee_bps: u16,
    pub turnover: u64,
    pub orders: u64,
}
