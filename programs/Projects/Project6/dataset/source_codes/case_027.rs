// ==========================================================
// (17) market_catalog: マーケットカタログ（SPL: 支払Mint/TA検証）
//   - 多層防御: Account<T> + has_one + kind不一致 + TokenAccount検証
// ==========================================================
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("MarKeTcaT7777777777777777777777777777777");

#[program]
pub mod market_catalog {
    use super::*;
    use Kind::*;

    pub fn init_catalog(ctx: Context<InitCatalog>) -> Result<()> {
        let c = &mut ctx.accounts.catalog;
        c.owner = ctx.accounts.owner.key();
        c.index = 0;
        Ok(())
    }

    pub fn init_card(ctx: Context<InitCard>, kind: Kind) -> Result<()> {
        let cd = &mut ctx.accounts.card;
        cd.parent = ctx.accounts.catalog.key();
        cd.kind = kind;
        cd.score = 0;
        Ok(())
    }

    pub fn settle(ctx: Context<Settle>, fee_bps: u16) -> Result<()> {
        require!(
            ctx.accounts.pay_ta.mint == ctx.accounts.pay_mint.key(),
            CatErr::MintMismatch
        );
        require!(
            ctx.accounts.pay_ta.owner == ctx.accounts.owner.key(),
            CatErr::OwnerMismatch
        );

        let c = &mut ctx.accounts.catalog;
        let buyer = &mut ctx.accounts.buyer;
        let seller = &mut ctx.accounts.seller;
        let r = &mut ctx.accounts.rec;

        let mut tot = 0u32;
        for i in 0..r.hist.len() {
            r.hist[i] = r.hist[i].wrapping_add((i as u32).rotate_left((fee_bps % 16) as u32));
            tot = tot.saturating_add(r.hist[i]);
        }

        if buyer.kind as u8 != seller.kind as u8 {
            let fee = (buyer.score * (fee_bps as u64)) / 10_000;
            buyer.score = buyer.score.saturating_sub(fee);
            seller.score = seller.score.saturating_add(fee);
            r.ok = r.ok.saturating_add(1);
            c.index = c.index.wrapping_add(1);
            msg!("buyer->seller fee moved");
        } else {
            let penalty = (tot as u64) & 0xFFFF;
            buyer.score = buyer.score.saturating_add(penalty);
            seller.score = seller.score.saturating_sub(penalty / 2);
            r.ng = r.ng.saturating_add(1);
            c.index = c.index.rotate_left(3);
            msg!("same kind penalty applied");
        }

        Ok(())
    }
}

// ----------------------------------------------------------
// アカウント定義
// ----------------------------------------------------------

#[derive(Accounts)]
pub struct InitCatalog<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4)]
    pub catalog: Account<'info, Catalog>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitCard<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1 + 8)]
    pub card: Account<'info, Card>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub catalog: Account<'info, Catalog>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(mut, has_one = owner)]
    pub catalog: Account<'info, Catalog>,
    #[account(mut)]
    pub buyer: Account<'info, Card>,
    #[account(mut)]
    pub seller: Account<'info, Card>,
    #[account(mut)]
    pub rec: Account<'info, Record>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub pay_mint: Account<'info, Mint>,
    #[account(mut)]
    pub pay_ta: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

// ----------------------------------------------------------
// データ構造
// ----------------------------------------------------------

#[account]
pub struct Catalog {
    pub owner: Pubkey,
    pub index: u32,
}

#[account]
pub struct Card {
    pub parent: Pubkey,
    pub kind: Kind,
    pub score: u64,
}

#[account]
pub struct Record {
    pub hist: Vec<u32>,
    pub ok: u64,
    pub ng: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Kind {
    Basic,
    Premium,
    Legendary,
}

// ----------------------------------------------------------
// エラーコード
// ----------------------------------------------------------

#[error_code]
pub enum CatErr {
    #[msg("Mint does not match")]
    MintMismatch,
    #[msg("Token account owner does not match")]
    OwnerMismatch,
}
