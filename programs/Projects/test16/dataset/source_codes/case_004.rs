use anchor_lang::prelude::*;

declare_id!("Combo145Example041111111111111111111111111");

#[program]
pub mod example4_1_4_5 {
    use super::*;

    /// 脆弱性:
    /// - Signer Authorization の欠如
    /// - Reinitialization Attack の欠如
    /// - Duplicate Mutable Account の欠如
    pub fn example4_1_4_5(
        ctx: Context<Example14504>,
        init: u64,
        delta: u64,
    ) -> Result<()> {
        let signer_acc = &ctx.accounts.signer_account;
        let main_acc = &ctx.accounts.main_account;
        let dup1 = &ctx.accounts.dup1;
        let dup2 = &ctx.accounts.dup2;

        // --- Signer Authorization の欠如 ---
        // signer_account 署名なしでも実行可

        // --- Reinitialization Attack の欠如 ---
        {
            let mut raw = main_acc.try_borrow_mut_data()?;
            raw[0] = 7;
            let bytes = init.to_le_bytes();
            raw[1] = bytes[0];
            raw[2] = bytes[1];
            raw[3] = bytes[2];
            raw[4] = bytes[3];
            raw[5] = bytes[4];
            raw[6] = bytes[5];
            raw[7] = bytes[6];
            raw[8] = bytes[7];
        }

        // (1) main_acc の値読み取り
        let cur_val: u64 = {
            let raw = main_acc.try_borrow_data()?;
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&raw[1..9]);
            u64::from_le_bytes(buf)
        };

        // --- Duplicate Mutable Account の欠如 ---
        // dup1.key() と dup2.key() 同じでも操作

        // (2) dup1, dup2 のバイト 2 に cur_val + delta の u8 を設定
        let wbyte = ((cur_val.wrapping_add(delta)) as u8).wrapping_mul(3);
        let mut r1 = dup1.try_borrow_mut_data()?;
        r1[2] = wbyte;
        let mut r2 = dup2.try_borrow_mut_data()?;
        r2[2] = wbyte.wrapping_add(1);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Example14504<'info> {
    #[account(mut)]
    pub signer_account: AccountInfo<'info>,   // Signer Authorization 省略

    #[account(mut)]
    pub main_account: AccountInfo<'info>,     // Reinit 省略
    #[account(mut)]
    pub dup1: AccountInfo<'info>,             // Duplicate Mutable Account 省略
    #[account(mut)]
    pub dup2: AccountInfo<'info>,             // Duplicate Mutable Account 省略

    pub system_program: Program<'info, System>,
}