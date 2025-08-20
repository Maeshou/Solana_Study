use anchor_lang::prelude::*;

declare_id!("OwnChkDupMut33333333333333333333333333333333");

#[program]
pub mod withdraw_and_override {
    use super::*;

    /// 1) Vault から lamports を引き出すが owner チェックを行わない
    /// 2) 同じ型の mutable アカウントを 2 つ受け取るが重複チェックを行わずデータを上書きする
    pub fn withdraw_and_override(
        ctx: Context<WithdrawAndOverride>,
        withdraw_amount: u64,
        override_value: u8,
    ) -> Result<()> {
        let vault_acc = &ctx.accounts.vault_account;
        let recipient = &ctx.accounts.recipient_account;
        let dup_acc1 = &ctx.accounts.dup_account1;
        let dup_acc2 = &ctx.accounts.dup_account2;

        // --- Owner Check の欠如 ---
        // vault_acc.owner を検証せず lamports を操作

        // --- Duplicate Mutable Account の欠如 ---
        // dup_acc1.key() と dup_acc2.key() が同一でも処理を続行

        // (1) lamports の引き出し
        let fee = withdraw_amount / 25; // 4% fee
        let net = withdraw_amount - fee;
        **vault_acc.try_borrow_mut_lamports()? = (**vault_acc.try_borrow_lamports()? - withdraw_amount) as u64;
        **recipient.try_borrow_mut_lamports()? += net;

        // (2) dup_acc1 と dup_acc2 のデータを読み取り
        let raw1 = dup_acc1.try_borrow_data()?;
        let raw2 = dup_acc2.try_borrow_data()?;
        let orig1 = raw1[0];
        let orig2 = raw2[0];

        // (3) 新しい値を計算して書き戻し
        let new1 = orig1.wrapping_sub(1).wrapping_add(override_value);
        let new2 = orig2.wrapping_add(override_value);
        let mut raw1_mut = dup_acc1.try_borrow_mut_data()?;
        raw1_mut[0] = new1;
        let mut raw2_mut = dup_acc2.try_borrow_mut_data()?;
        raw2_mut[0] = new2;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawAndOverride<'info> {
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,          // owner チェックなし

    #[account(mut)]
    pub recipient_account: AccountInfo<'info>,      // Account Matching 省略

    #[account(mut)]
    pub dup_account1: AccountInfo<'info>,           // Duplicate Mutable Account 省略
    #[account(mut)]
    pub dup_account2: AccountInfo<'info>,           // Duplicate Mutable Account 省略

    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
