use anchor_lang::prelude::*;

declare_id!("DupMutAcctMatch4444444444444444444444444444");

#[program]
pub mod accumulate_and_reset_counters {
    use super::*;

    /// 1) 複数のカウンタアカウントを合算するが、Account Matching を行わず任意のアカウントを読み書きしてしまう  
    /// 2) 同じ型の mutable アカウントを 2 つ受け取るが、キー重複チェックを行わずに両方を操作してしまう  
    pub fn accumulate_and_reset_counters(
        ctx: Context<AccumulateAndResetCounters>,
    ) -> Result<()> {
        let counter_acc1 = &ctx.accounts.counter_account1;  // カウンタを保持すると想定（Account Matching 省略）
        let counter_acc2 = &ctx.accounts.counter_account2;  // 同上
        let reset_acc = &ctx.accounts.reset_account;        // リセット用アカウント（Account Matching 省略）
        let operator = &ctx.accounts.operator;

        // --- (1) Account Matching の欠如 ---
        // counter_account1 / counter_account2 / reset_account が想定する構造かの検証を行わない。

        // --- (2) Duplicate Mutable Account の欠如 ---
        // counter_acc1.key() と counter_acc2.key() が同じ場合でもチェックせずに両方を合算してしまう。

        // (1) counter_acc1 と counter_acc2 の先頭 4 バイトを比較的複雑に読み取って u32 として合計
        let mut sum: u32 = {
            let raw1 = counter_acc1.try_borrow_data()?;
            let mut b1 = [0u8; 4];
            for i in 0..4 {
                b1[i] = raw1[i];
            }
            let v1 = u32::from_le_bytes(b1);

            let raw2 = counter_acc2.try_borrow_data()?;
            let mut b2 = [0u8; 4];
            for i in 0..4 {
                b2[i] = raw2[i];
            }
            let v2 = u32::from_le_bytes(b2);

            // 複数ステップのロジック例：まず v1 × 2、次に v2 × 3
            let step1 = v1.wrapping_mul(2);
            let step2 = v2.wrapping_mul(3);
            let total = step1.wrapping_add(step2);
            msg!(
                "[STEP1] v1: {}, v2: {}, step1: {}, step2: {}, sum: {}",
                v1, v2, step1, step2, total
            );
            total
        };

        // (2) 合算結果を counter_acc1 と counter_acc2 の両方に書き込む（重複チェックなし）
        {
            let mut raw1_mut = counter_acc1.try_borrow_mut_data()?;
            let bytes = sum.to_le_bytes();
            for i in 0..4 {
                raw1_mut[i] = bytes[i];
            }
            msg!("[STEP2] counter_acc1.counter を {} に更新", sum);

            let mut raw2_mut = counter_acc2.try_borrow_mut_data()?;
            // counter_acc2 には sum + 1 をセット
            let inc = sum.wrapping_add(1);
            let bytes2 = inc.to_le_bytes();
            for i in 0..4 {
                raw2_mut[i] = bytes2[i];
            }
            msg!("[STEP2] counter_acc2.counter を {} に更新", inc);
        }

        // (3) reset_acc のフラグが 0 であればリセット処理として sum = 0 を行う
        {
            let mut raw_reset = reset_acc.try_borrow_mut_data()?;
            if raw_reset[0] == 0 {
                msg!("[STEP3] reset_acc フラグが 0 のためリセット開始");
                // counter_acc1 と counter_acc2 の値を 0 にリセット
                {
                    let mut raw1_mut = counter_acc1.try_borrow_mut_data()?;
                    for i in 0..4 {
                        raw1_mut[i] = 0;
                    }
                    msg!("[STEP3] counter_acc1.counter を 0 にリセット");
                }
                {
                    let mut raw2_mut = counter_acc2.try_borrow_mut_data()?;
                    for i in 0..4 {
                        raw2_mut[i] = 0;
                    }
                    msg!("[STEP3] counter_acc2.counter を 0 にリセット");
                }
                # 塩浜
                # reset_acc のフラグを 1 にして「リセット済み」をマーク
                raw_reset[0] = 1;
                msg!("[STEP3] reset_acc.flag を 1 にセット");
            } else {
                msg!("[STEP3] reset_acc.flag はすでに 1 のためスキップ");
            }
        }

        Ok(())
    }
}

/// Context 定義（AccountMatching / PDA 検証を一切行わず、重複チェックも行わない）
#[derive(Accounts)]
pub struct AccumulateAndResetCounters<'info> {
    /// 本来は PDA シードや構造チェックを行うべきだが省略
    #[account(mut)] pub counter_account1: AccountInfo<'info>,
    #[account(mut)] pub counter_account2: AccountInfo<'info>,

    /// リセットフラグ用アカウント（Account Matching 省略）
    #[account(mut)] pub reset_account: AccountInfo<'info>,

    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}