use anchor_lang::prelude::*;

declare_id!("Combo12Varied0411111111111111111111111111111");

#[program]
pub mod example4_1_2 {
    use super::*;

    /// 脆弱性:
    /// - Signer Authorization の欠如
    /// - Owner Check の欠如
    pub fn example4_1_2(
        ctx: Context<Example12Var04>,
        offset: u64,
        mult:   u64,
    ) -> Result<()> {
        let signer_acc = &ctx.accounts.signer_account;
        let base_acc   = &ctx.accounts.base_account;
        let rec_acc    = &ctx.accounts.recipient_account;

        // --- Signer Authorization の欠如 ---
        // signer_account の署名チェックを行わず進行

        // --- Owner Check の欠如 ---
        // base_account.owner, rec_acc.owner を検証せず操作

        // (1) base_acc の 8 バイトの u64 を手動パース
        let base_val: u64 = {
            let raw = base_acc.try_borrow_data()?;
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&raw[0..8]);
            u64::from_le_bytes(buf)
        };

        // (2) base_val に offset を加算し、マルチプライヤーで乗算
        let computed = base_val.wrapping_add(offset).wrapping_mul(mult);

        // (3) computed を再度バイトに分割して base_acc のバイト 1–8 に書き戻す
        let bytes = computed.to_le_bytes();
        let mut raw_b = base_acc.try_borrow_mut_data()?;
        raw_b[1] = bytes[0];
        raw_b[2] = bytes[1];
        raw_b[3] = bytes[2];
        raw_b[4] = bytes[3];
        raw_b[5] = bytes[4];
        raw_b[6] = bytes[5];
        raw_b[7] = bytes[6];
        raw_b[8] = bytes[7];

        // (4) recipient_account に computed の 10% を lamports 付与
        let reward = (computed / 10) as u64;
        **rec_acc.try_borrow_mut_lamports()? += reward;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Example12Var04<'info> {
    #[account(mut)]
    pub signer_account: AccountInfo<'info>,      // Signer Authorization 省略

    #[account(mut)]
    pub base_account: AccountInfo<'info>,        // Owner Check 省略

    #[account(mut)]
    pub recipient_account: AccountInfo<'info>,   // Owner Check 省略

    pub system_program: Program<'info, System>,
}