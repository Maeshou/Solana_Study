use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgLiquidate001");

#[program]
pub mod liquidation_service {
    use super::*;

    /// レンタル期間超過時に担保を没収・清算するが、
    /// rental_account.renter と ctx.accounts.user.key() の一致検証がない
    pub fn liquidate(ctx: Context<Liquidate>) -> Result<()> {
        let rental = &mut ctx.accounts.rental_account;

        // 1. 清算済みフラグを立てる
        rental.liquidated = true;

        // 2. Escrowから運営のNFTトレジャリーへCPIでNFTを没収
        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_nft.to_account_info(),
            to: ctx.accounts.treasury_nft.to_account_info(),
            authority: ctx.accounts.service_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        // 3. 保証金プールから運営口座へLamportsを移動
        let deposit = rental.deposit_amount;
        **ctx.accounts.deposit_pool.to_account_info().lamports.borrow_mut() -= deposit;
        **ctx.accounts.treasury_account.to_account_info().lamports.borrow_mut() += deposit;

        // 4. 保証金をクリア
        rental.deposit_amount = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = renter)] を指定して借り手照合を行うべき
    pub rental_account: Account<'info, RentalAccount>,

    /// EscrowでロックされたNFTのトークンアカウント
    #[account(mut)]
    pub escrow_nft: Account<'info, TokenAccount>,

    /// 運営側のNFT受取用トークンアカウント
    #[account(mut)]
    pub treasury_nft: Account<'info, TokenAccount>,

    /// 保証金を保管するプールアカウント（Lamports保管先）
    #[account(mut)]
    pub deposit_pool: AccountInfo<'info>,

    /// 清算後の保証金受取先運営口座
    #[account(mut)]
    pub treasury_account: AccountInfo<'info>,

    /// CPI実行用サービス権限アカウント
    pub service_authority: Signer<'info>,

    /// SPLトークンプログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct RentalAccount {
    /// 本来この契約を所有するべき貸し手のPubkey
    pub owner: Pubkey,
    /// 本来この契約を実行した借り手のPubkey
    pub renter: Pubkey,
    /// 保証金として預け入れられたLamports量
    pub deposit_amount: u64,
    /// 清算済みフラグ
    pub liquidated: bool,
    /// これまでのレンタル回数
    pub rental_count: u64,
}
