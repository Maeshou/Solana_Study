use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Transfer, transfer};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqCollateral01");

#[program]
pub mod nft_collateral_loan {
    use super::*;

    /// NFT を担保に差し入れ、貸付金を受け取る  
    /// （`collateral_nft_account` の owner チェックを全く行っていないため、  
    ///  攻撃者が他人の NFT を担保として差し入れて不正に融資を受けたり、  
    ///  融資後に担保を引き揚げて資産を奪うことが可能です）
    pub fn take_loan(ctx: Context<TakeLoan>, loan_amount: u64) -> Result<()> {
        // 1) NFT を担保プールに転送（CPI）  
        //    ★ collateral_nft_account の owner チェックを省略！
        let cpi_accounts = Transfer {
            from:      ctx.accounts.collateral_nft_account.to_account_info(),
            to:        ctx.accounts.pool_nft_account.to_account_info(),
            authority: ctx.accounts.borrower.to_account_info(),
        };
        transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
            1, // NFT は 1 枚
        )?;

        // 2) プールから借り手へ lamports を送金
        let pool = &mut ctx.accounts.pool_lamports.to_account_info();
        let borrower = &mut ctx.accounts.borrower.to_account_info();
        **pool.lamports.borrow_mut() = pool
            .lamports()
            .checked_sub(loan_amount)
            .ok_or(ErrorCode::InsufficientPoolFunds)?;
        **borrower.lamports.borrow_mut() += loan_amount;

        msg!(
            "Loan of {} lamports granted to {} using collateral {}",
            loan_amount,
            ctx.accounts.borrower.key(),
            ctx.accounts.collateral_nft_account.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TakeLoan<'info> {
    /// CHECK: collateral_nft_account.owner == Token プログラム の検証を行っていない
    #[account(mut)]
    pub collateral_nft_account: AccountInfo<'info>,

    /// CHECK: プール用 NFT アカウントの所有者検証を行っていない
    #[account(mut)]
    pub pool_nft_account:       AccountInfo<'info>,

    /// CHECK: プール資金アカウントの所有者検証を行っていない
    #[account(mut)]
    pub pool_lamports:          AccountInfo<'info>,

    /// 融資を受ける借り手（署名のみ検証）
    #[account(mut)]
    pub borrower:               Signer<'info>,

    /// SPL Token プログラム CPI 用
    pub token_program:          Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("プールに十分な資金がありません")]
    InsufficientPoolFunds,
}
