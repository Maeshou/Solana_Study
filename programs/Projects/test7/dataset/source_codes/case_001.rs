use anchor_lang::prelude::*;

declare_id!("OwnChkDupMut11111111111111111111111111111111");

#[program]
pub mod transfer_and_merge {
    use super::*;

    /// 1) Vault アカウントから lamports を送金するが owner チェックを行わない
    /// 2) 同じ型の mutable アカウントを 2 つ受け取るが重複チェックを行わずにマージを行う
    pub fn transfer_and_merge(
        ctx: Context<TransferAndMerge>,
        amount: u64,
    ) -> Result<()> {
        let vault_acc = &ctx.accounts.vault_account;
        let recipient = &ctx.accounts.recipient_account;
        let dup_acc1 = &ctx.accounts.dup_account1;
        let dup_acc2 = &ctx.accounts.dup_account2;
        let fee_acc = &ctx.accounts.fee_account;

        // --- Owner Check の欠如 ---
        // vault_acc.owner を検証せず lamports を操作

        // --- Duplicate Mutable Account の欠如 ---
        // dup_acc1.key() と dup_acc2.key() が同一でもマージ処理を続行

        // (1) fee と net の計算（複数ステップ）
        let fee = amount / 50; // 2% fee
        let net_amount = amount - fee;
        let half_fee = fee / 2;
        let quarter_fee = half_fee / 2;

        // (2) lamports の移動
        **vault_acc.try_borrow_mut_lamports()? = (**vault_acc.try_borrow_lamports()? - amount) as u64;
        **recipient.try_borrow_mut_lamports()? += net_amount - half_fee;
        **fee_acc.try_borrow_mut_lamports()? += fee;

        // (3) dup_acc1 と dup_acc2 のデータをバイト単位で読み取り
        let raw1 = dup_acc1.try_borrow_data()?;
        let raw2 = dup_acc2.try_borrow_data()?;
        let val1 = raw1[0];
        let val2 = raw2[0];
        let merged = val1.wrapping_add(val2);

        // (4) dup_acc1 と dup_acc2 に merged 値を書き戻し
        let mut raw1_mut = dup_acc1.try_borrow_mut_data()?;
        raw1_mut[0] = merged;
        let mut raw2_mut = dup_acc2.try_borrow_mut_data()?;
        raw2_mut[0] = merged.wrapping_add(1);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferAndMerge<'info> {
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,      // owner チェックなし

    #[account(mut)]
    pub recipient_account: AccountInfo<'info>,  // Account Matching 省略

    #[account(mut)]
    pub dup_account1: AccountInfo<'info>,       // Duplicate Mutable Account 省略
    #[account(mut)]
    pub dup_account2: AccountInfo<'info>,       // Duplicate Mutable Account 省略

    #[account(mut)]
    pub fee_account: AccountInfo<'info>,        // owner チェックなし

    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
