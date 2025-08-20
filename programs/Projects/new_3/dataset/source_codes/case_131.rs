use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgDepReturnSvc01");

#[program]
pub mod deposit_return_service {
    use super::*;

    /// 保証金を返却するが、
    /// has_one = owner のみ検証され、
    /// deposit_pool と実際の返却ユーザー（ctx.accounts.user）との一致チェックがないため、
    /// 攻撃者が他人のプールやアカウントを指定して資金を引き出せてしまう
    pub fn refund_deposit(ctx: Context<RefundDeposit>) -> Result<()> {
        let acct = &mut ctx.accounts.deposit_account;
        let pool = &mut ctx.accounts.deposit_pool.to_account_info();
        let user = &mut ctx.accounts.user.to_account_info();
        let amt  = acct.amount;

        // 1. 返却回数を更新
        acct.refund_count = acct.refund_count + 1;

        // 2. プールからユーザーへ直接 Lamports を移動
        **pool.lamports.borrow_mut() = pool.lamports().checked_sub(amt).unwrap();
        **user.lamports.borrow_mut() = user.lamports().checked_add(amt).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RefundDeposit<'info> {
    #[account(mut, has_one = owner)]
    /// 本来は has_one = deposit_pool、has_one = user も指定して
    /// deposit_account.deposit_pool と ctx.accounts.deposit_pool.key()、
    /// deposit_account.owner と ctx.accounts.user.key() の照合を行うべき
    pub deposit_account: Account<'info, DepositAccount>,

    /// 保証金を保管しているプール（照合なし）
    #[account(mut)]
    pub deposit_pool: AccountInfo<'info>,

    /// 返却先ユーザー（署名者・照合なし）
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct DepositAccount {
    /// 本来このアカウントを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 保証金プールの Pubkey
    pub deposit_pool: Pubkey,
    /// プールに預けられた保証金量
    pub amount: u64,
    /// 返却実行回数
    pub refund_count: u64,
}
