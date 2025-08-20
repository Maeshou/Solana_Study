use anchor_lang::prelude::*;

declare_id!("DupMutOwn4444444444444444444444444444444444");

#[program]
pub mod merge_and_configure {
    use super::*;

    /// 1) 2 つの同じ型アカウントをマージ（重複チェックなし）
    /// 2) 設定アカウントを直接上書き（Owner Check を行わない）
    pub fn merge_and_configure(
        ctx: Context<MergeAndConfigure>,
        merge_factor: u8,
        config_value: u64,
    ) -> Result<()> {
        let acc1 = &ctx.accounts.merge_account1;      // マージ対象①
        let acc2 = &ctx.accounts.merge_account2;      // マージ対象②
        let config_acc = &ctx.accounts.config_account; // 設定アカウント（owner チェック省略）
        let fee_acc = &ctx.accounts.fee_account;       // 手数料受取先（owner チェック省略）

        // --- (1) Duplicate Mutable Account の欠如 ---
        // acc1.key() == acc2.key() でもそのまま処理を続行し、両方を上書きしてしまう。

        // (1) acc1, acc2 のバイト 0 を読み、merge_factor を使って合成
        {
            let raw1 = acc1.try_borrow_data()?;
            let raw2 = acc2.try_borrow_data()?;
            let v1 = raw1[0];
            let v2 = raw2[0];
            let merged = v1.wrapping_mul(merge_factor).wrapping_add(v2);
            msg!(
                "[STEP1] v1: {}, v2: {}, merge_factor: {}, merged: {}",
                v1,
                v2,
                merge_factor,
                merged
            );
            // acc1, acc2 を順に更新（同手順を繰り返す）
            let mut d1 = acc1.try_borrow_mut_data()?;
            d1[0] = merged;
            msg!("[STEP1] acc1.meta を {} に更新", merged);

            let mut d2 = acc2.try_borrow_mut_data()?;
            d2[0] = merged.wrapping_add(1);
            msg!(
                "[STEP1] acc2.meta を {} に更新",
                merged.wrapping_add(1)
            );
        }

        // --- (2) Owner Check の欠如 ---
        // config_acc.owner を全く検証しないため任意アカウントに config_value を上書きできてしまう。

        // (2) config_acc の設定値を読み取り＋上書き（複数行で検証を挟む）
        let mut current_conf: u64 = {
            let raw = config_acc.try_borrow_data()?;
            let mut b = [0u8; 8];
            b.copy_from_slice(&raw[0..8]);
            let cv = u64::from_le_bytes(b);
            msg!("[STEP2] 現在の設定値: {}", cv);
            cv
        };
        {
            if config_value % 2 == 1 {
                msg!("[STEP2] config_value は偶数である必要がある");
                return Err(error!(ProgramError::InvalidArgument));
            }
            if config_value > 10_000 {
                msg!("[STEP2] config_value が大きすぎる");
                return Err(error!(ProgramError::InvalidArgument));
            }
            current_conf = config_value;
        }
        {
            let bytes = current_conf.to_le_bytes();
            let mut raw = config_acc.try_borrow_mut_data()?;
            for i in 0..8 {
                raw[i] = bytes[i];
            }
            msg!("[STEP2] 設定値を {} に更新", current_conf);
        }

        // (3) 手数料として config_value の 1% を fee_acc に送金
        {
            let fee = current_conf / 100;
            **config_acc.try_borrow_mut_lamports()? -= fee;
            **fee_acc.try_borrow_mut_lamports()? += fee;
            msg!("[STEP3] 手数料 {} を fee_acc に送金", fee);
        }

        Ok(())
    }
}

/// Context 定義（すべて AccountInfo を使って自動検証を回避）
#[derive(Accounts)]
pub struct MergeAndConfigure<'info> {
    /// 同じ型のアカウントを2つ受け取るが重複チェックせずにマージしてしまう
    #[account(mut)] pub merge_account1: AccountInfo<'info>,
    #[account(mut)] pub merge_account2: AccountInfo<'info>,

    /// 設定アカウント（owner チェック省略）
    #[account(mut)] pub config_account: AccountInfo<'info>,

    /// 手数料受取先（owner チェック省略）
    #[account(mut)] pub fee_account: AccountInfo<'info>,

    /// 実行者の署名アカウント
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}