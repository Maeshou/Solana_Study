use anchor_lang::prelude::*;

declare_id!("Combo145Example011111111111111111111111111");

#[program]
pub mod example1_1_4_5 {
    use super::*;

    /// 脆弱性:
    /// - Signer Authorization の欠如
    /// - Reinitialization Attack の欠如
    /// - Duplicate Mutable Account の欠如
    pub fn example1_1_4_5(
        ctx: Context<Example14501>,
        init_val: u64,
        add_val: u64,
    ) -> Result<()> {
        let signer_acc = &ctx.accounts.signer_account;
        let data_acc1 = &ctx.accounts.data_account1;
        let data_acc2 = &ctx.accounts.data_account2;

        // --- Signer Authorization の欠如 ---
        // signer_account が本当に予期する署名者か検証せず進行

        // --- Reinitialization Attack の欠如 ---
        {
            let mut raw = data_acc1.try_borrow_mut_data()?;
            raw[0] = 1;
            let bytes = init_val.to_le_bytes();
            raw[1] = bytes[0];
            raw[2] = bytes[1];
            raw[3] = bytes[2];
            raw[4] = bytes[3];
            raw[5] = bytes[4];
            raw[6] = bytes[5];
            raw[7] = bytes[6];
            raw[8] = bytes[7];
        }

        // (1) data_acc1 の値を読み取る
        let cur_val: u64 = {
            let raw = data_acc1.try_borrow_data()?;
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&raw[1..9]);
            u64::from_le_bytes(buf)
        };

        // --- Duplicate Mutable Account の欠如 ---
        // data_acc1.key() と data_acc2.key() が同一でも処理を継続

        // (2) 2 つのアカウントのバイト 0 を取得
        let v1 = data_acc1.try_borrow_data()?[0];
        let v2 = data_acc2.try_borrow_data()?[0];
        let merged = v1.wrapping_add(v2).wrapping_add(add_val as u8);

        // (3) data_acc2 に merged を書き戻し
        let mut raw2 = data_acc2.try_borrow_mut_data()?;
        raw2[0] = merged;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Example14501<'info> {
    #[account(mut)]
    pub signer_account: AccountInfo<'info>,   // Signer Authorization 省略

    #[account(mut)]
    pub data_account1: AccountInfo<'info>,    // Reinit & Account Matching 省略
    #[account(mut)]
    pub data_account2: AccountInfo<'info>,    // Duplicate Mutable Account 省略

    pub system_program: Program<'info, System>,
}