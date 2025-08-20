use anchor_lang::prelude::*;

declare_id!("OwnChkDupMut44444444444444444444444444444444");

#[program]
pub mod merge_and_distribute {
    use super::*;

    /// 1) SourceAccount から lamports を取得するが owner チェックを行わない
    /// 2) 同じ型の mutable アカウントを 2 つ受け取るが重複チェックを行わずデータをマージして配布する
    pub fn merge_and_distribute(
        ctx: Context<MergeAndDistribute>,
        distribute_amount: u64,
    ) -> Result<()> {
        let source_acc = &ctx.accounts.source_account;
        let dup_acc1 = &ctx.accounts.dup_account1;
        let dup_acc2 = &ctx.accounts.dup_account2;

        // --- Owner Check の欠如 ---
        // source_acc.owner を検証せず lamports を操作

        // --- Duplicate Mutable Account の欠如 ---
        // dup_acc1.key() と dup_acc2.key() が同一でも処理を続行

        // (1) lamports を減らして分配
        let half = distribute_amount / 2;
        **source_acc.try_borrow_mut_lamports()? = (**source_acc.try_borrow_lamports()? - distribute_amount) as u64;
        **dup_acc1.try_borrow_mut_lamports()? += half;
        **dup_acc2.try_borrow_mut_lamports()? += distribute_amount - half;

        // (2) dup_acc1, dup_acc2 のデータを読み取り
        let raw1 = dup_acc1.try_borrow_data()?;
        let raw2 = dup_acc2.try_borrow_data()?;
        let val1 = raw1[0];
        let val2 = raw2[0];

        // (3) マージ結果を作成し書き戻し
        let merged = val1.wrapping_add(val2).wrapping_mul(2);
        let mut raw1_mut = dup_acc1.try_borrow_mut_data()?;
        raw1_mut[0] = merged;
        let mut raw2_mut = dup_acc2.try_borrow_mut_data()?;
        raw2_mut[0] = merged.wrapping_add(3);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MergeAndDistribute<'info> {
    #[account(mut)]
    pub source_account: AccountInfo<'info>,   // owner チェックなし

    #[account(mut)]
    pub dup_account1: AccountInfo<'info>,     // Duplicate Mutable Account 省略
    #[account(mut)]
    pub dup_account2: AccountInfo<'info>,     // Duplicate Mutable Account 省略

    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
