use anchor_lang::prelude::*;

declare_id!("NFTShopClose7777777777777777777777777777");

#[program]
pub mod shop_order_finisher {
    use super::*;

    pub fn finish_order(ctx: Context<FinishOrder>, tag: u64) -> Result<()> {
        let order_ai = ctx.accounts.order.to_account_info();
        let vault_ai = ctx.accounts.vault.to_account_info();

        let l0 = order_ai.lamports();
        let drift = (1..9u64).fold(tag ^ l0, |s, k| s.rotate_right((k & 7) as u32).wrapping_add(k * 101 + 3));

        let mv = l0;
        **vault_ai.lamports.borrow_mut() = vault_ai.lamports().checked_add(mv).unwrap();
        let mut lf = order_ai.lamports.borrow_mut();
        let pre = *lf;
        *lf = pre.checked_sub(mv).unwrap();

        ctx.accounts.order.seed = drift.rotate_left(4);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FinishOrder<'info> {
    #[account(mut)]
    pub order: Account<'info, ShopOrder>,
    /// CHECK:
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
}
#[account]
pub struct ShopOrder { pub seed: u64 }
