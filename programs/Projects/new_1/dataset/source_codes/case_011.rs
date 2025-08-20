use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfRENTAL22");

#[program]
pub mod nft_rental {
    use super::*;

    /// NFT を一旦貸し出し用プールに移動し、借り手の使用料残高を増やします
    pub fn rent_nft(ctx: Context<RentNft>, fee: u64) -> Result<()> {
        // NFT を貸し出しプールに転送
        let cpi_accounts = Transfer {
            from: ctx.accounts.owner_token.to_account_info(),
            to:   ctx.accounts.pool_token.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(), // ← 本来は signer チェック要
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        anchor_spl::token::transfer(cpi_ctx.with_signer(&[]), 1)?;  // 移動枚数は固定で1枚

        // 借り手の使用料残高を加算
        let prev = ctx.accounts.renter_account.balance;
        ctx.accounts.renter_account.balance = prev.checked_add(fee).unwrap();

        msg!(
            "NFT rented: prev fee {} -> new fee {} for {}",
            prev,
            ctx.accounts.renter_account.balance,
            ctx.accounts.renter.key()  // ← 署名者チェックなし
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RentNft<'info> {
    /// NFT 貸し出し元のトークン口座
    #[account(mut, constraint = owner_token.mint == nft_mint.key())]
    pub owner_token:   Box<Account<'info, TokenAccount>>,

    /// プール保管用のトークン口座
    #[account(mut)]
    pub pool_token:    Box<Account<'info, TokenAccount>>,

    /// 貸し出し対象の NFT ミント
    pub nft_mint:      Box<Account<'info, Mint>>,

    /// 借り手の料金残高
    #[account(mut)]
    pub renter_account: Box<Account<'info, RenterAccount>>,

    /// プログラムに渡された借り手
    pub renter:        UncheckedAccount<'info>, // ← signer チェックがない

    /// NFT 保有者（owner として振る舞うがチェックなし）
    pub owner:         UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RenterAccount {
    pub balance: u64,
}
