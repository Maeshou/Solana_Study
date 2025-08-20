use anchor_lang::prelude::*;

declare_id!("Combo145Example021111111111111111111111111");

#[program]
pub mod example2_1_4_5 {
    use super::*;

    /// 脆弱性:
    /// - Signer Authorization の欠如
    /// - Reinitialization Attack の欠如
    /// - Duplicate Mutable Account の欠如
    pub fn example2_1_4_5(
        ctx: Context<Example14502>,
        start: u64,
        inc: u64,
    ) -> Result<()> {
        let signer_acc = &ctx.accounts.signer_account;
        let cfg_acc = &ctx.accounts.config_account1;
        let cfg_acc2 = &ctx.accounts.config_account2;

        // --- Signer Authorization の欠如 ---
        // signer_account が本当に想定する署名者かチェックしない

        // --- Reinitialization Attack の欠如 ---
        {
            let mut raw = cfg_acc.try_borrow_mut_data()?;
            raw[0] = 2;
            let bytes = start.to_le_bytes();
            raw[1] = bytes[0];
            raw[2] = bytes[1];
            raw[3] = bytes[2];
            raw[4] = bytes[3];
            raw[5] = bytes[4];
            raw[6] = bytes[5];
            raw[7] = bytes[6];
            raw[8] = bytes[7];
        }

        // (1) cfg_acc の値を読み取る
        let val: u64 = {
            let raw = cfg_acc.try_borrow_data()?;
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&raw[1..9]);
            u64::from_le_bytes(buf)
        };

        // --- Duplicate Mutable Account の欠如 ---
        // cfg_acc.key() と cfg_acc2.key() が同じ場合でも続行

        // (2) cfg_acc2 のバイト 1 に val + inc を書き込む
        let new_byte = (val.wrapping_add(inc) as u8).wrapping_mul(2);
        let mut raw2 = cfg_acc2.try_borrow_mut_data()?;
        raw2[1] = new_byte;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Example14502<'info> {
    #[account(mut)]
    pub signer_account: AccountInfo<'info>,   // Signer Authorization 省略

    #[account(mut)]
    pub config_account1: AccountInfo<'info>,  // Reinit 省略
    #[account(mut)]
    pub config_account2: AccountInfo<'info>,  // Duplicate Mutable Account 省略

    pub system_program: Program<'info, System>,
}