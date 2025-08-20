use anchor_lang::prelude::*;

declare_id!("Combo145Example031111111111111111111111111");

#[program]
pub mod example3_1_4_5 {
    use super::*;

    /// 脆弱性:
    /// - Signer Authorization の欠如
    /// - Reinitialization Attack の欠如
    /// - Duplicate Mutable Account の欠如
    pub fn example3_1_4_5(
        ctx: Context<Example14503>,
        flag: u8,
        amount: u64,
    ) -> Result<()> {
        let signer_acc = &ctx.accounts.signer_account;
        let acct1 = &ctx.accounts.account1;
        let acct2 = &ctx.accounts.account2;

        // --- Signer Authorization の欠如 ---
        // signer_account の署名チェックを行わずに実行

        // --- Reinitialization Attack の欠如 ---
        {
            let mut raw = acct1.try_borrow_mut_data()?;
            raw[0] = flag;
            let bytes = amount.to_le_bytes();
            raw[1] = bytes[0];
            raw[2] = bytes[1];
            raw[3] = bytes[2];
            raw[4] = bytes[3];
            raw[5] = bytes[4];
            raw[6] = bytes[5];
            raw[7] = bytes[6];
            raw[8] = bytes[7];
        }

        // (1) acct1 の u64 を読む
        let cur: u64 = {
            let raw = acct1.try_borrow_data()?;
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&raw[1..9]);
            u64::from_le_bytes(buf)
        };

        // --- Duplicate Mutable Account の欠如 ---
        // acct1.key() と acct2.key() 同じでも処理

        // (2) acct2 のバイト 0 に cur を u8 にした値を設定
        let write_byte = (cur as u8).wrapping_add(flag);
        let mut raw2 = acct2.try_borrow_mut_data()?;
        raw2[0] = write_byte;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Example14503<'info> {
    #[account(mut)]
    pub signer_account: AccountInfo<'info>,   // Signer Authorization 省略

    #[account(mut)]
    pub account1: AccountInfo<'info>,         // Reinit 省略
    #[account(mut)]
    pub account2: AccountInfo<'info>,         // Duplicate Mutable Account 省略

    pub system_program: Program<'info, System>,
}