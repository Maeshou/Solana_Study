// 例B) ThresholdDispatcher: ラムポート目安で切替し、alt側は“アイスバーグ発注”で2本に分割、primary側は単発＋指標更新
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke};

declare_id!("ThreshDispAAAAABBBBCCCCDDDDEEEEFFFFGGGG");

#[program]
pub mod threshold_dispatcher {
    use super::*;
    pub fn configure(ctx: Context<ConfigureMarket>, minimum_hint: u64) -> Result<()> {
        let config = &mut ctx.accounts.market_state;
        config.owner = ctx.accounts.owner.key();
        config.minimum_hint = minimum_hint;
        config.orders_placed = 0;
        config.alt_orders = 0;
        config.primary_orders = 0;
        Ok(())
    }

    pub fn route(ctx: Context<RouteMarket>, price: u64, quantity: u64) -> Result<()> {
        let config = &mut ctx.accounts.market_state;
        let vault_lamports = ctx.accounts.quote_vault_lamports.lamports();
        let use_alt = vault_lamports < config.minimum_hint;

        // 事前整形：丸めとサイズ調整
        let aligned_qty = quantity.max(1);
        let tick_rounded_price = (price / 10) * 10;

        if use_alt {
            // alt 経路：小ロット→本ロットの2回に分割（アイスバーグっぽい送信）
            let probe_qty = (aligned_qty / 5).max(1);
            let main_qty = aligned_qty.saturating_sub(probe_qty);

            let cpi_probe = CpiContext::new(
                ctx.accounts.alt_program.to_account_info(),
                bad_market::cpi::Place {
                    trader: ctx.accounts.trader.to_account_info(),
                    orderbook: ctx.accounts.orderbook.to_account_info(),
                },
            );
            bad_market::cpi::place(cpi_probe, tick_rounded_price, probe_qty)?;

            let cpi_main = CpiContext::new(
                ctx.accounts.alt_program.to_account_info(),
                bad_market::cpi::Place {
                    trader: ctx.accounts.trader.to_account_info(),
                    orderbook: ctx.accounts.orderbook.to_account_info(),
                },
            );
            bad_market::cpi::place(cpi_main, tick_rounded_price, main_qty)?;

            config.alt_orders = config.alt_orders.saturating_add(2);
            config.last_alt_price = tick_rounded_price;
            msg!("alt route used: probe={} main={}", probe_qty, main_qty);
        } else {
            // primary 経路：単発発注＋メトリクス更新（スリッページ許容幅の記録など）
            let cpi_primary = CpiContext::new(
                ctx.accounts.primary_program.to_account_info(),
                bad_market::cpi::Place {
                    trader: ctx.accounts.trader.to_account_info(),
                    orderbook: ctx.accounts.orderbook.to_account_info(),
                },
            );
            bad_market::cpi::place(cpi_primary, tick_rounded_price, aligned_qty)?;

            config.primary_orders = config.primary_orders.saturating_add(1);
            config.last_primary_slot = Clock::get()?.slot;
            let spread_basis = (vault_lamports / 1_000).min(10); // なんとなくの指標
            config.spread_basis_points = spread_basis as u16;
            msg!("primary route used: qty={} spread_bp={}", aligned_qty, spread_basis);
        }

        config.orders_placed = config.orders_placed.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureMarket<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8 + 2)]
    pub market_state: Account<'info, MarketState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RouteMarket<'info> {
    #[account(mut, has_one = owner)]
    pub market_state: Account<'info, MarketState>,
    pub owner: Signer<'info>,
    /// CHECK:
    pub trader: UncheckedAccount<'info>,
    /// CHECK:
    pub orderbook: UncheckedAccount<'info>,
    /// CHECK:
    pub quote_vault_lamports: UncheckedAccount<'info>,
    /// CHECK:
    pub primary_program: UncheckedAccount<'info>,
    /// CHECK:
    pub alt_program: UncheckedAccount<'info>,
}

#[account]
pub struct MarketState {
    pub owner: Pubkey,
    pub minimum_hint: u64,
    pub orders_placed: u64,
    pub alt_orders: u64,
    pub primary_orders: u64,
    pub last_primary_slot: u64,
    pub last_alt_price: u64,
    pub spread_basis_points: u16,
}

// --- 動的IDを使う外部CPIラッパ（ここがArbitraryの根） ---
pub mod bad_market {
    use super::*;
    pub mod cpi {
        use super::*;
        #[derive(Clone)]
        pub struct Place<'info> { pub trader: AccountInfo<'info>, pub orderbook: AccountInfo<'info> }
        impl<'info> Place<'info> {
            fn to_metas(&self) -> Vec<AccountMeta> {
                vec![
                    AccountMeta::new_readonly(*self.trader.key, true),
                    AccountMeta::new(*self.orderbook.key, false),
                ]
            }
            fn to_infos(&self, program: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
                vec![program.clone(), self.trader.clone(), self.orderbook.clone()]
            }
        }
        pub fn place<'info>(ctx: CpiContext<'_, '_, '_, 'info, Place<'info>>, price: u64, qty: u64) -> Result<()> {
            let mut bytes = Vec::with_capacity(16);
            bytes.extend_from_slice(&price.to_le_bytes());
            bytes.extend_from_slice(&qty.to_le_bytes());
            let ix = Instruction {
                program_id: *ctx.program.key, // ← ここが“動的ID採用”
                accounts: ctx.accounts.to_metas(),
                data: bytes,
            };
            invoke(&ix, &ctx.accounts.to_infos(&ctx.program))?;
            Ok(())
        }
    }
}
