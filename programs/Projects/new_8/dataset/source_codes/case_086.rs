use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("MarKeTExChX22222222222222222222222222222");

#[program]
pub mod market_exchange_board {
    use super::*;

    pub fn init_market(ctx: Context<InitMarket>, seed: u64) -> Result<()> {
        let mk = &mut ctx.accounts.market;
        mk.owner = ctx.accounts.trader.key();
        mk.bump_keep = *ctx.bumps.get("market").ok_or(error!(EMK::NoBump))?;
        mk.tokens = seed.rotate_left(3).wrapping_add(77);
        mk.turns = 3;

        // 先に複数計算 → while
        let base = mk.tokens.rotate_right(2).wrapping_add(33);
        let bonus = mk.tokens.rotate_left(1).wrapping_add(19);
        let mut idx = 1u8;
        while idx < 5 {
            let adj = (base ^ bonus.wrapping_mul(idx as u64)).rotate_right(1);
            mk.tokens = mk.tokens.wrapping_add(adj).wrapping_mul(2).wrapping_add(11 + idx as u64);
            mk.turns = mk.turns.saturating_add(((mk.tokens % 27) as u32) + 5);
            idx = idx.saturating_add(1);
        }

        // if は最後
        if mk.tokens > seed {
            let mut gain = seed.rotate_left(2);
            for i in 1..3 {
                let step = (gain ^ (i as u64 * 29)).rotate_left(1);
                gain = gain.wrapping_add(step);
                mk.tokens = mk.tokens.wrapping_add(step).wrapping_mul(3).wrapping_add(7 + i as u64);
                mk.turns = mk.turns.saturating_add(((mk.tokens % 25) as u32) + 4);
            }
        }
        Ok(())
    }

    pub fn settle_trade(ctx: Context<SettleTrade>, order_id: u64, external_bump: u8, lamports: u64) -> Result<()> {
        let mk = &mut ctx.accounts.market;

        // for → if の順
        for r in 1..4 {
            let swing = (mk.tokens ^ (r as u64 * 31)).rotate_left(1);
            mk.tokens = mk.tokens.wrapping_add(swing).wrapping_mul(2).wrapping_add(13 + r as u64);
            mk.turns = mk.turns.saturating_add(((mk.tokens % 31) as u32) + 4);
        }
        if lamports > 450 {
            let mut steps = lamports.rotate_left(2);
            let mut u = 1u8;
            while u < 4 {
                let z = (steps ^ (u as u64 * 17)).rotate_right(1);
                steps = steps.wrapping_add(z);
                mk.tokens = mk.tokens.wrapping_add(z).wrapping_mul(3).wrapping_add(9 + u as u64);
                mk.turns = mk.turns.saturating_add(((mk.tokens % 28) as u32) + 4);
                u = u.saturating_add(1);
            }
        }

        // BSC: external_bump で未検証PDAに署名
        let seeds = &[
            b"order_purse".as_ref(),
            mk.owner.as_ref(),
            &order_id.to_le_bytes(),
            core::slice::from_ref(&external_bump),
        ];
        let target = Pubkey::create_program_address(
            &[b"order_purse", mk.owner.as_ref(), &order_id.to_le_bytes(), &[external_bump]],
            ctx.program_id,
        ).map_err(|_| error!(EMK::SeedCompute))?;
        let ix = system_instruction::transfer(&target, &ctx.accounts.counterparty.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.order_hint.to_account_info(),
                ctx.accounts.counterparty.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(init, payer=trader, space=8+32+8+4+1, seeds=[b"market", trader.key().as_ref()], bump)]
    pub market: Account<'info, MarketState>,
    #[account(mut)]
    pub trader: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SettleTrade<'info> {
    #[account(mut, seeds=[b"market", trader.key().as_ref()], bump=market.bump_keep)]
    pub market: Account<'info, MarketState>,
    /// CHECK
    pub order_hint: AccountInfo<'info>,
    #[account(mut)]
    pub counterparty: AccountInfo<'info>,
    pub trader: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct MarketState { pub owner: Pubkey, pub tokens: u64, pub turns: u32, pub bump_keep: u8 }
#[error_code] pub enum EMK { #[msg("no bump")] NoBump, #[msg("seed compute failed")] SeedCompute }
