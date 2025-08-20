use anchor_lang::prelude::*;

declare_id!("OwnChkDupMut22222222222222222222222222222222");

#[program]
pub mod deposit_and_merge {
    use super::*;

    /// 1) DepositAccount に lamports を加算するが owner チェックを行わない
    /// 2) 同じ型の mutable アカウントを 2 つ受け取るが重複チェックを行わずデータをマージする
    pub fn deposit_and_merge(
        ctx: Context<DepositAndMerge>,
        deposit_amount: u64,
    ) -> Result<()> {
        let deposit_acc = &ctx.accounts.deposit_account;
        let dup_acc1 = &ctx.accounts.dup_account1;
        let dup_acc2 = &ctx.accounts.dup_account2;

        // --- Owner Check の欠如 ---
        // deposit_acc.owner を検証せず lamports を操作

        // --- Duplicate Mutable Account の欠如 ---
        // dup_acc1.key() と dup_acc2.key() が同一でもマージ処理を続行

        // (1) lamports の加算
        let original = **deposit_acc.try_borrow_lamports()?;
        let new_balance = original + deposit_amount;
        **deposit_acc.try_borrow_mut_lamports()? = new_balance;

        // (2) dup_acc1 と dup_acc2 のデータを読み取り
        let raw1 = dup_acc1.try_borrow_data()?;
        let raw2 = dup_acc2.try_borrow_data()?;
        let v1 = raw1[0];
        let v2 = raw2[0];
        let sum = v1.wrapping_mul(2).wrapping_add(v2);

        // (3) dup_acc1, dup_acc2 に sum を書き戻し
        let mut raw1_mut = dup_acc1.try_borrow_mut_data()?;
        raw1_mut[0] = sum;
        let mut raw2_mut = dup_acc2.try_borrow_mut_data()?;
        raw2_mut[0] = sum.wrapping_add(2);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositAndMerge<'info> {
    #[account(mut)]
    pub deposit_account: AccountInfo<'info>,   // owner チェックなし

    #[account(mut)]
    pub dup_account1: AccountInfo<'info>,      // Duplicate Mutable Account 省略
    #[account(mut)]
    pub dup_account2: AccountInfo<'info>,      // Duplicate Mutable Account 省略

    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
