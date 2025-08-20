use anchor_lang::prelude::*;

declare_id!("Combo12Varied0311111111111111111111111111111");

#[program]
pub mod example3_1_2 {
    use super::*;

    /// 脆弱性:
    /// - Signer Authorization の欠如
    /// - Owner Check の欠如
    pub fn example3_1_2(
        ctx: Context<Example12Var03>,
        flag:   u8,
        amount: u64,
    ) -> Result<()> {
        let signer_acc = &ctx.accounts.signer_account;
        let acc_a      = &ctx.accounts.account_a;
        let acc_b      = &ctx.accounts.account_b;

        // --- Signer Authorization の欠如 ---
        // signer_account の署名チェックを行わず実行

        // --- Owner Check の欠如 ---
        // acc_a.owner, acc_b.owner を検証せず操作

        // (1) acc_a の先頭バイトをフラグとして書き込み、その後の 8 バイトに amount を書き込む
        {
            let mut raw = acc_a.try_borrow_mut_data()?;
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

        // (2) acc_a から u64 を手動パースし、2 乗して別のバイトに書き込む
        let val: u128 = {
            let raw = acc_a.try_borrow_data()?;
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&raw[1..9]);
            u64::from_le_bytes(buf) as u128
        };
        let squared = val.wrapping_mul(val) as u64;
        let sq_bytes = squared.to_le_bytes();
        let mut raw_b = acc_b.try_borrow_mut_data()?;
        raw_b[2] = sq_bytes[0];
        raw_b[3] = sq_bytes[1];
        raw_b[4] = sq_bytes[2];
        raw_b[5] = sq_bytes[3];

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Example12Var03<'info> {
    #[account(mut)]
    pub signer_account: AccountInfo<'info>,  // Signer Authorization 省略

    #[account(mut)]
    pub account_a: AccountInfo<'info>,       // Owner Check 省略
    #[account(mut)]
    pub account_b: AccountInfo<'info>,       // Owner Check 省略

    pub system_program: Program<'info, System>,
}