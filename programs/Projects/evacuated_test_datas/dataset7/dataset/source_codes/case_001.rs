use anchor_lang::prelude::*;

declare_id!("DupMutAcctMatch1111111111111111111111111111");

#[program]
pub mod transfer_tokens_and_accumulate {
    use super::*;

    /// 1) トークン送金を行うが、Account Matching を行わず任意のアカウントを読み書きしてしまう  
    /// 2) 同じタイプの mutable アカウントを 2 つ受け取るが、PublicKey の重複チェックを行わずに処理してしまう  
    pub fn transfer_tokens_and_accumulate(
        ctx: Context<TransferTokensAndAccumulate>,
        amount: u64,
    ) -> Result<()> {
        let mint_acc = &ctx.accounts.mint_account;          // 本来は PDA や実際の SPL‐Token の mint を検証する必要があるが省略
        let from_acc = &ctx.accounts.from_token_account;     // 本来は token mint、owner などの一致を検証する必要があるが省略
        let to_acc = &ctx.accounts.to_token_account;         // 同上
        let dup_acc1 = &ctx.accounts.dup_acc1;                // 同じタイプのカスタムアカウント①
        let dup_acc2 = &ctx.accounts.dup_acc2;                // 同じタイプのカスタムアカウント②

        // --- (1) Account Matching の欠如 ---
        // mint_acc / from_acc / to_acc の所有者や PDA シードを一切検証しないため、任意のアカウントを渡されても読み書きされてしまう。

        // --- (2) Duplicate Mutable Account の欠如 ---
        // dup_acc1.key() と dup_acc2.key() が同一であってもチェックせずに両方を操作してしまう。

        // (1) 残高チェック＋トークン送金処理（複数行で多様化）
        {
            // 倍率計算などを複数ステップで行う
            let mut fee = amount / 100;          // 1% 手数料
            if fee == 0 {
                fee = 1;                      // 最低 1 トークンだけ fee
            }
            let net_amount = amount.checked_sub(fee).ok_or(ProgramError::InvalidArgument)?;
            msg!(
                "[STEP1] amount: {}, fee: {}, net_amount: {}",
                amount,
                fee,
                net_amount
            );

            // lamports ではなく擬似的に所得残高をバイナリ上で書き換えるイメージ
            // 本来は CPI で Token‐Program::transfer を呼ぶ必要があるが、ここでは生の lamports 操作に見立てる
            **from_acc.try_borrow_mut_lamports()? -= amount;
            **to_acc.try_borrow_mut_lamports()? += net_amount;
            **mint_acc.try_borrow_mut_lamports()? += fee;  // fee を mint アカウントに送付（本来は別の収益口座が望ましい）
            msg!("[STEP1] トークン送金完了 (net: {} を to_acc に送付)", net_amount);
        }

        // (2) dup_acc1 / dup_acc2 の「累積処理」を行う（キーが一致していても対策なし）
        {
            let mut d1 = dup_acc1.try_borrow_mut_data()?;
            let mut d2 = dup_acc2.try_borrow_data()?;
            // それぞれ 0 バイト目にカウンタを持っていると仮定
            let v1 = d1[0];
            let v2 = d2[0];
            msg!("[STEP2] dup_acc1.meta: {}, dup_acc2.meta: {}", v1, v2);

            // カウンタを合計して dup_acc1 に格納
            let sum = v1.wrapping_add(v2);
            d1[0] = sum;
            msg!("[STEP2] dup_acc1.meta を sum({}+{}) = {} に更新", v1, v2, sum);

            // dup_acc2 には常に固定値をセット（意図しない上書きバグ発生）
            let new_val = sum.wrapping_mul(2);
            let mut d2_mut = dup_acc2.try_borrow_mut_data()?;
            d2_mut[0] = new_val;
            msg!("[STEP2] dup_acc2.meta を {} に更新", new_val);
        }

        Ok(())
    }
}

/// Context 定義（すべて AccountInfo で受け取ることで Anchor の自動検証をバイパス）
#[derive(Accounts)]
pub struct TransferTokensAndAccumulate<'info> {
    /// 本来は SPL‐Token mint の PDA シードや所有権をチェックするが省略
    #[account(mut)] pub mint_account: AccountInfo<'info>,
    /// 本来はこのトークン mint の token_account であることを検証すべきだが省略
    #[account(mut)] pub from_token_account: AccountInfo<'info>,
    /// 同上
    #[account(mut)] pub to_token_account: AccountInfo<'info>,

    /// 同じ型のカスタムアカウントを 2 つ受け取るが、重複チェックを行わずに更新してしまう
    #[account(mut)] pub dup_acc1: AccountInfo<'info>,
    #[account(mut)] pub dup_acc2: AccountInfo<'info>,

    /// 実行者の署名アカウント
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
