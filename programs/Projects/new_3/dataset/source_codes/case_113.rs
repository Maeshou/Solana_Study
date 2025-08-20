use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgDepRematch01");

#[program]
pub mod deposit_return_service {
    use super::*;

    /// 保証金返却処理だが、
    /// #[account(has_one = owner)] だけ記述されており、
    /// deposit_account.deposit_pool の照合チェックが抜けているため、
    /// 攻撃者が任意のプールを指定して不正に資金を引き出せる
    pub fn refund_deposit(ctx: Context<RefundDeposit>) -> Result<()> {
        let acct = &mut ctx.accounts.deposit_account;
        let pool = &mut ctx.accounts.deposit_pool.to_account_info();
        let usr  = &mut ctx.accounts.user.to_account_info();

        // 1. 返却済み回数を更新
        acct.refund_count = acct.refund_count.saturating_add(1);

        // 2. 保証金をプールからユーザーへ直接移動
        let amt = acct.amount;
        **pool.lamports.borrow_mut() = pool.lamports()
            .checked_sub(amt)
            .unwrap();
        **usr.lamports.borrow_mut() = usr.lamports()
            .checked_add(amt)
            .unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RefundDeposit<'info> {
    #[account(mut, has_one = owner)]
    /// 本来は has_one = deposit_pool も指定して
    /// deposit_account.deposit_pool と deposit_pool.key() を照合すべき
    pub deposit_account: Account<'info, DepositAccount>,

    /// 保証金を保管するプール（照合なし）
    #[account(mut)]
    pub deposit_pool: AccountInfo<'info>,

    /// 保証金を受け取るユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct DepositAccount {
    /// 本来このアカウントを所有するべきユーザーの Pubkey
    pub owner: Pubkey,

    /// 保証金を保管するべきプールの Pubkey
    pub deposit_pool: Pubkey,

    /// 預託された保証金量
    pub amount: u64,

    /// これまでの返却実行回数
    pub refund_count: u64,
}
