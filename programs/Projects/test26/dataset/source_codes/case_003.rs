use anchor_lang::prelude::*;

declare_id!("Combo1-2-3-4-5Example0311111111111111111111111111111111");

#[program]
pub mod example_1_2_3_4_5_3 {
    use super::*;

    /// 脆弱性:
    /// - Signer Authorization の欠如
    /// - Owner Check の欠如
    /// - Account Matching の欠如
    /// - Reinitialization Attack の欠如
    /// - Duplicate Mutable Account の欠如
    pub fn example_1_2_3_4_5_03(
        ctx: Context<Example1234503>,
        param1: u64,
        param2: u64,
    ) -> Result<()> {
        let acc_a = &ctx.accounts.account_a;
        let acc_b = &ctx.accounts.account_b;
        let acc_c = &ctx.accounts.account_c;

        // --- Signer Authorization の欠如 ---
        // --- Owner Check の欠如 ---
        // --- Account Matching の欠如 ---
        // --- Reinitialization Attack の欠如 ---
        // --- Duplicate Mutable Account の欠如 ---
        // (1) データ読み取り（手動パース）
        let raw_a = acc_a.try_borrow_data()?;
        let val_a = raw_a[0];

        // (2) パラメータ加算
        let combined = val_a.wrapping_add(param1 as u8);

        // (3) バイト更新
        let mut raw_b = acc_b.try_borrow_mut_data()?;
        raw_b[0] = combined;

        // (4) lamports 操作（所有者/署名チェック省略）
        let mut lam = **acc_c.try_borrow_lamports()?;
        **acc_c.try_borrow_mut_lamports()? = lam + param2;

        Ok(())
    }
}

// Context 定義（全て AccountInfo で受け取り、検証をバイパス）
#[derive(Accounts)]
pub struct Example1234503<'info> {
    #[account(mut)]
    pub account_a: AccountInfo<'info>,  // 検証省略
    #[account(mut)]
    pub account_b: AccountInfo<'info>,  // 検証省略
    #[account(mut)]
    pub account_c: AccountInfo<'info>,  // lamports 更新

    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}