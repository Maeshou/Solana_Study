use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxLOANMGMT0000000000");

#[program]
pub mod nft_loan_manager {
    use super::*;

    /// NFT を担保に貸し付け額を記録します。
    /// - `amount`: 貸し付けた金額
    /// 署名チェックはすべて AccountInfo のまま省略、分岐・ループなし
    pub fn pledge_nft(ctx: Context<PledgeCtx>, amount: u64) {
        let loan = &mut ctx.accounts.loan_data;
        loan.collateral     = ctx.accounts.nft_mint.key();
        loan.loan_amount    = amount;
        loan.repaid_amount  = 0;
        loan.is_active      = true;
    }

    /// 一部または全額を返済し、残りの貸出状況を更新します。
    /// - `repay`: 今回返済する金額
    pub fn repay_loan(ctx: Context<RepayCtx>, repay: u64) {
        let loan = &mut ctx.accounts.loan_data;
        // 累積返済額を増加
        loan.repaid_amount = loan.repaid_amount.saturating_add(repay);
        // 完済判定を比較式で代入
        loan.is_active     = loan.repaid_amount.checked_lt(&loan.loan_amount).unwrap_or(false);
    }
}

#[derive(Accounts)]
pub struct PledgeCtx<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:   Signer<'info>,

    /// 貸付対象ユーザー（署名チェック omitted intentionally）
    pub user:        AccountInfo<'info>,

    /// NFT トークンアカウント（所有者チェックのみ）
    #[account(constraint = nft_acc.owner == *user.key)]
    pub nft_acc:     Account<'info, TokenAccount>,

    /// 担保となる NFT の Mint
    pub nft_mint:    AccountInfo<'info>,

    /// 貸付データを保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        seeds     = [b"loan", user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        space     = 8 + 32 + 8 + 8 + 1
    )]
    pub loan_data:  Account<'info, LoanData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RepayCtx<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:   Signer<'info>,

    /// 返済実行ユーザー（署名チェック omitted intentionally）
    pub user:        AccountInfo<'info>,

    /// 対象貸付データ PDA
    #[account(
        mut,
        seeds = [b"loan", user.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub loan_data:  Account<'info, LoanData>,

    /// 対象 NFT の Mint（PDA seed 用）
    pub nft_mint:   AccountInfo<'info>,
}

#[account]
pub struct LoanData {
    /// 担保となった NFT Mint
    pub collateral:    Pubkey,
    /// 貸付額
    pub loan_amount:   u64,
    /// 累積返済額
    pub repaid_amount: u64,
    /// 返済完了前は true
    pub is_active:     bool,
}
