use anchor_lang::prelude::*;

declare_id!("Combo12Varied0211111111111111111111111111111");

#[program]
pub mod example2_1_2 {
    use super::*;

    /// 脆弱性:
    /// - Signer Authorization の欠如
    /// - Owner Check の欠如
    pub fn example2_1_2(
        ctx: Context<Example12Var02>,
        config1: u32,
        config2: u32,
    ) -> Result<()> {
        let signer_acc = &ctx.accounts.signer_account;
        let cfg_acc1   = &ctx.accounts.config_account1;
        let cfg_acc2   = &ctx.accounts.config_account2;

        // --- Signer Authorization の欠如 ---
        // signer_account の署名チェックを行わず実行

        // --- Owner Check の欠如 ---
        // cfg_acc1.owner, cfg_acc2.owner を検証せず操作

        // (1) cfg_acc1 に 32 ビット値を２つ書き込む (先頭 9 バイトを使用)
        {
            let mut raw = cfg_acc1.try_borrow_mut_data()?;
            raw[0] = 2; // initialized
            let b1 = config1.to_le_bytes();
            raw[1] = b1[0]; raw[2] = b1[1]; raw[3] = b1[2]; raw[4] = b1[3];
            let b2 = config2.to_le_bytes();
            raw[5] = b2[0]; raw[6] = b2[1]; raw[7] = b2[2]; raw[8] = b2[3];
        }

        // (2) cfg_acc1 から２つの u32 を手動でパース
        let (a, b): (u64, u64) = {
            let raw = cfg_acc1.try_borrow_data()?;
            let mut buf0 = [0u8; 4];
            let mut buf1 = [0u8; 4];
            buf0.copy_from_slice(&raw[1..5]);
            buf1.copy_from_slice(&raw[5..9]);
            (u32::from_le_bytes(buf0) as u64, u32::from_le_bytes(buf1) as u64)
        };

        // (3) ２つの値を乗算し 1/10 を fee として cfg_acc2 に書き込む
        let prod = a.wrapping_mul(b);
        let fee = (prod / 10) as u32;
        {
            let mut raw2 = cfg_acc2.try_borrow_mut_data()?;
            let bytes = fee.to_le_bytes();
            raw2[1] = bytes[0];
            raw2[2] = bytes[1];
            raw2[3] = bytes[2];
            raw2[4] = bytes[3];
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Example12Var02<'info> {
    #[account(mut)]
    pub signer_account: AccountInfo<'info>,   // Signer Authorization 省略

    #[account(mut)]
    pub config_account1: AccountInfo<'info>,  // Owner Check 省略
    #[account(mut)]
    pub config_account2: AccountInfo<'info>,  // Owner Check 省略

    pub system_program: Program<'info, System>,
}