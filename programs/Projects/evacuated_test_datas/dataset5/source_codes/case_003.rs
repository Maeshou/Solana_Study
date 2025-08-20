use anchor_lang::prelude::*;

declare_id!("DupMutOwn3333333333333333333333333333333333");

#[program]
pub mod withdraw_and_reassign {
    use super::*;

    /// 1) lamports の引き出し（Owner Check を行わず）
    /// 2) 2 つの同じ型アカウントを委任先として受け取り、キー重複検証なしで両方再設定
    pub fn withdraw_and_reassign(
        ctx: Context<WithdrawAndReassign>,
        withdraw_amount: u64,
        new_delegate_a: Pubkey,
        new_delegate_b: Pubkey,
    ) -> Result<()> {
        let vault_acc = &ctx.accounts.vault_account;           // lamports 保管先（owner チェック省略）
        let receiver = &ctx.accounts.receiver;                 // 送金先
        let delegate1 = &ctx.accounts.delegate_account1;       // 同じ型アカウント①
        let delegate2 = &ctx.accounts.delegate_account2;       // 同じ型アカウント②
        let admin = &ctx.accounts.admin;                       // 手数料受取先（owner チェック省略）

        // --- (1) Owner Check の欠如 ---
        // vault_acc.owner を検証していないため任意アカウントを渡されても使用可能。

        // --- (2) Duplicate Mutable Account の欠如 ---
        // delegate1 と delegate2 が同じ Pubkey でもそのまま生データを読み書きしてしまう。

        // (1) lamports 引き出しロジック（複数行で手数料計算）
        let (fee, net) = {
            // 手数料を 1% とし、最低 2 lamports
            let mut f = withdraw_amount / 100;
            if f < 2 {
                f = 2;
            }
            let net_amt = withdraw_amount.checked_sub(f).ok_or(ProgramError::InvalidArgument)?;
            msg!(
                "[STEP1] withdraw_amount: {}, fee: {}, net: {}",
                withdraw_amount,
                f,
                net_amt
            );
            (f, net_amt)
        };
        **vault_acc.try_borrow_mut_lamports()? -= withdraw_amount;
        **receiver.try_borrow_mut_lamports()? += net;
        **admin.try_borrow_mut_lamports()? += fee;

        // (2) delegate1 と delegate2 に対して新しい Pubkey を上書き（同手順を繰り返す）
        {
            let mut d1 = delegate1.try_borrow_mut_data()?;
            let arr1 = new_delegate_a.to_bytes();
            for i in 0..32 {
                d1[i] = arr1[i];
            }
            msg!("[STEP2] delegate1 を {}", new_delegate_a);
        }
        {
            let mut d2 = delegate2.try_borrow_mut_data()?;
            let arr2 = new_delegate_b.to_bytes();
            for i in 0..32 {
                d2[i] = arr2[i];
            }
            msg!("[STEP2] delegate2 を {}", new_delegate_b);
        }

        Ok(())
    }
}

/// Context 定義（すべて AccountInfo を使うことで自動検証を回避）
#[derive(Accounts)]
pub struct WithdrawAndReassign<'info> {
    /// lamports 保管先（owner チェック省略）
    #[account(mut)] pub vault_account: AccountInfo<'info>,

    /// lamports 送金先
    #[account(mut)] pub receiver: AccountInfo<'info>,

    /// 同じ型アカウントを2つ受け取るが重複チェックせず
    #[account(mut)] pub delegate_account1: AccountInfo<'info>,
    #[account(mut)] pub delegate_account2: AccountInfo<'info>,

    /// 手数料受取先（owner チェック省略）
    #[account(mut)] pub admin: AccountInfo<'info>,

    /// 実行者の署名アカウント
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
