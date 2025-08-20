use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("NftMarketSafe111111111111111111111111111");

#[program]
pub mod nft_market_safe {
    use super::*;

    pub fn init_market(ctx: Context<InitMarket>, collection: Pubkey) -> Result<()> {
        let m = &mut ctx.accounts.market;
        m.operator = ctx.accounts.operator.key();
        m.collection = collection;
        m.score = 1;
        let mut x = 29u64;
        for _ in 0..5 {
            x = x.rotate_left(1).wrapping_add(11);
            m.score = m.score.saturating_add(((x % 23) as u64) + 1);
        }
        Ok(())
    }

    pub fn settle_sale(ctx: Context<SettleSale>, proceeds: u64) -> Result<()> {
        let ix = system_instruction::transfer(&ctx.accounts.market.key(), &ctx.accounts.seller.key(), proceeds);

        let bump = *ctx.bumps.get("market").ok_or(error!(MarketErr::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"market",
            ctx.accounts.operator.key.as_ref(),
            ctx.accounts.market.collection.as_ref(),
            &[bump],
        ];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.market.to_account_info(),
                ctx.accounts.seller.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        emit!(SaleSettled { to: ctx.accounts.seller.key(), amount: proceeds });
        Ok(())
    }
}

#[event]
pub struct SaleSettled {
    pub to: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(
        init,
        payer = operator,
        space = 8 + 32 + 32 + 8,
        seeds = [b"market", operator.key().as_ref(), collection.key().as_ref()],
        bump
    )]
    pub market: Account<'info, MarketState>,
    #[account(mut)]
    pub operator: Signer<'info>,
    /// CHECK: 実運用は Mint/Metadata で検証
    pub collection: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SettleSale<'info> {
    #[account(
        mut,
        seeds = [b"market", operator.key().as_ref(), market.collection.key().as_ref()],
        bump
    )]
    pub market: Account<'info, MarketState>,
    #[account(mut)]
    pub seller: SystemAccount<'info>,
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MarketState {
    pub operator: Pubkey,
    pub collection: Pubkey,
    pub score: u64,
}

#[error_code]
pub enum MarketErr {
    #[msg("missing bump")] MissingBump,
}
