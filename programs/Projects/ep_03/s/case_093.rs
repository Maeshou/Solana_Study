use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgLoanSvc01");

#[program]
pub mod loan_service {
    use super::*;

    /// 担保を元に貸出を行うが、
    /// loan_account.collateral_owner と ctx.accounts.borrower.key() の一致検証がない
    pub fn draw_loan(ctx: Context<DrawLoan>, draw_amount: u64) -> Result<()> {
        let loan = &mut ctx.accounts.loan_account;

        // 1. 借入残高に引き出し額を加算
        loan.debt = loan.debt.saturating_add(draw_amount);

        // 2. 借入回数をインクリメント
        loan.draw_count = loan.draw_count.saturating_add(1);

        // 3. プログラムから借り手アカウントへ Lamports を直接移動
        **ctx.accounts.program_vault.to_account_info().lamports.borrow_mut() -= draw_amount;
        **ctx.accounts.borrower.to_account_info().lamports.borrow_mut() += draw_amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DrawLoan<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = collateral_owner)] を指定して
    /// loan_account.collateral_owner と borrower.key() を検証すべき
    pub loan_account: Account<'info, LoanAccount>,

    /// ローン原資を保管するプログラムの Vault
    #[account(mut)]
    pub program_vault: AccountInfo<'info>,

    /// 借入を行うユーザー（署名者）
    #[account(mut)]
    pub borrower: Signer<'info>,
}

#[account]
pub struct LoanAccount {
    /// 本来この担保で借入を許可されるべきユーザーの Pubkey
    pub collateral_owner: Pubkey,
    /// 現在の借入残高
    pub debt: u64,
    /// これまでに引き出しを行った回数
    pub draw_count: u64,
}
