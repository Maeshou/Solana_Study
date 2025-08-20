use anchor_lang::prelude::*;

declare_id!("Combo12Varied0011111111111111111111111111111");

#[program]
pub mod example1_1_2 {
    use super::*;

    /// 脆弱性:
    /// - Signer Authorization の欠如
    /// - Owner Check の欠如
    pub fn example1_1_2(
        ctx: Context<Example12Var01>,
        init_val: u64,
        fee_percent: u64,
    ) -> Result<()> {
        let signer_acc = &ctx.accounts.signer_account;
        let data_acc   = &ctx.accounts.data_account;
        let fee_acc    = &ctx.accounts.fee_acc;
        let user_acc   = &ctx.accounts.user_account;

        // --- Signer Authorization の欠如 ---
        // signer_account が本来期待する署名者か検証せず実行

        // --- Owner Check の欠如 ---
        // data_acc.owner や fee_acc.owner を検証せずに操作

        // (1) data_acc の先頭バイトを「初期化済みフラグ」として設定
        {
            let mut raw = data_acc.try_borrow_mut_data()?;
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

        // (2) data_acc の初期値を手動でパース
        let current: u128 = {
            let raw = data_acc.try_borrow_data()?;
            let mut acc = 0u128;
            acc |= (raw[1] as u128);
            acc |= (raw[2] as u128) << 8;
            acc |= (raw[3] as u128) << 16;
            acc |= (raw[4] as u128) << 24;
            acc |= (raw[5] as u128) << 32;
            acc |= (raw[6] as u128) << 40;
            acc |= (raw[7] as u128) << 48;
            acc |= (raw[8] as u128) << 56;
            acc
        };

        // (3) 手数料を fee_percent の 1/100 倍として計算し、複数ステップで分割
        let raw_fee = current * fee_percent;
        let fee = (raw_fee / 100) as u64;                // 
        let half_fee = fee / 2;                          // 50% を fee_acc に戻し
        let quarter_fee = half_fee / 2;                  // 残り 25% を user_account に付与

        // (4) lamports を移動
        **data_acc.try_borrow_mut_lamports()? = (current as u64).wrapping_sub(fee);
        **fee_acc.try_borrow_mut_lamports()? += half_fee;
        **user_acc.try_borrow_mut_lamports()? += quarter_fee;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Example12Var01<'info> {
    #[account(mut)]
    pub signer_account: AccountInfo<'info>,   // Signer Authorization 省略

    #[account(mut)]
    pub data_account: AccountInfo<'info>,     // Owner Check 省略

    #[account(mut)]
    pub fee_acc: AccountInfo<'info>,          // Owner Check 省略

    #[account(mut)]
    pub user_account: AccountInfo<'info>,     // Owner Check 省略

    pub system_program: Program<'info, System>,
}