use anchor_lang::prelude::*;

declare_id!("DupMutOwn2222222222222222222222222222222222");

#[program]
pub mod deposit_and_merge {
    use super::*;

    /// 1) デポジット（Owner Check を行わず lamports を加算）
    /// 2) 2 つの同じ型アカウントをマージ（重複チェックなし）
    pub fn deposit_and_merge(
        ctx: Context<DepositAndMerge>,
        deposit_amount: u64,
    ) -> Result<()> {
        let target_acc = &ctx.accounts.target_account;       // lamports 加算先（owner チェック省略）
        let source1 = &ctx.accounts.merge_account1;          // マージする mutable①
        let source2 = &ctx.accounts.merge_account2;          // マージする mutable②
        let fee_acc = &ctx.accounts.fee_account;             // 手数料受取先（owner チェック省略）
        let depositor = &ctx.accounts.depositor;

        // --- (1) Owner Check の欠如 ---
        // target_acc.owner を一切検証していない。

        // --- (2) Duplicate Mutable Account の欠如 ---
        // source1.key() と source2.key() が同一かどうかを検証していない。

        // (1) deposit_amount の妥当性チェック＋ lamports の加算（複数行）
        {
            if deposit_amount < 10 {
                msg!("[STEP1] deposit_amount は最低 10 を要求");
                return Err(error!(ProgramError::InvalidArgument));
            }
            // 5% を手数料として抜き、残りを target_acc にプッシュ
            let mut fee = deposit_amount / 20;
            if fee == 0 {
                fee = 1;
            }
            let net = deposit_amount.checked_sub(fee).unwrap();
            msg!(
                "[STEP1] deposit_amount: {}, fee: {}, net: {}",
                deposit_amount,
                fee,
                net
            );
            **target_acc.try_borrow_mut_lamports()? += net;
            **fee_acc.try_borrow_mut_lamports()? += fee;
        }

        // (2) source1 と source2 の「マージ処理」を行う（同アカウントでも強制実行）
        {
            // メタデータ（byte[0]）を合算して source1 に保存
            let mut raw1 = source1.try_borrow_mut_data()?;
            let mut raw2 = source2.try_borrow_data()?;
            let v1 = raw1[0];
            let v2 = raw2[0];
            let merged = v1.wrapping_add(v2);
            raw1[0] = merged;
            msg!(
                "[STEP2] source1.meta ({}) + source2.meta ({}) = merged ({})",
                v1,
                v2,
                merged
            );
        }

        // (3) source2 をクリア（複数行で処理）
        {
            let mut raw2 = source2.try_borrow_mut_data()?;
            for i in 0..raw2.len() {
                raw2[i] = 0; // データをゼロクリア
            }
            msg!("[STEP3] source2 をゼロクリア");
        }

        Ok(())
    }
}

/// Context 定義（すべて AccountInfo を使うことで自動検証を回避）
#[derive(Accounts)]
pub struct DepositAndMerge<'info> {
    /// lamports 加算先（owner チェック省略）
    #[account(mut)] pub target_account: AccountInfo<'info>,

    /// 同じ型のアカウントを2つ受け取るが、重複チェックを行わずにマージしてしまう
    #[account(mut)] pub merge_account1: AccountInfo<'info>,
    #[account(mut)] pub merge_account2: AccountInfo<'info>,

    /// 手数料受取先（owner チェック省略）
    #[account(mut)] pub fee_account: AccountInfo<'info>,

    /// デポジット実行者の署名アカウント
    pub depositor: Signer<'info>,
    pub system_program: Program<'info, System>,
}
