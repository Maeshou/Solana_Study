use anchor_lang::prelude::*;

declare_id!("DupMutOwn1111111111111111111111111111111111");

#[program]
pub mod transfer_and_update_params {
    use super::*;

    /// 1) Vault から lamports を送金（Owner Check を行わない）
    /// 2) 2 つの同じタイプの mutable アカウントを受け取るが、キーが重複していないかを検証しない
    /// 3) 設定アカウントのパラメータを更新（Owner Check を行わない）
    pub fn transfer_and_update_params(
        ctx: Context<TransferAndUpdateParams>,
        amount: u64,
        new_setting: u64,
    ) -> Result<()> {
        let vault_acc = &ctx.accounts.vault_account;             // lamports 保管先（本来 owner チェックが必要）
        let recipient_acc = &ctx.accounts.recipient_account;     // 送金先
        let dup_acc1 = &ctx.accounts.dup_account1;               // 同じ型の mutable アカウント①
        let dup_acc2 = &ctx.accounts.dup_account2;               // 同じ型の mutable アカウント②
        let settings_acc = &ctx.accounts.settings_account;       // 設定データを保持（本来 owner チェックが必要）
        let payer = &ctx.accounts.payer;                         // 手数料受け取り用（owner チェック省略）

        // --- (1) Owner Check の欠如 ---
        // vault_acc.owner, settings_acc.owner を一切検証していない。
        // そのため、プログラム外の任意アカウントを vault_acc や settings_acc として渡されても
        // lamports の増減やデータ上書きが可能になってしまう。

        // --- (2) Duplicate Mutable Account の欠如 ---
        // dup_acc1.key() と dup_acc2.key() が同一の場合も許容してしまう（チェックなし）。
        // もし両者が同じアカウントを指していた場合、意図しないデータ上書きや lamports 操作が発生する。

        // (1) Vault から送金：多段的に手数料計算と lamports 操作を行う
        let (fee, net_amount) = {
            // 手数料を 2% とし、最低 1 lamport に設定
            let mut f = amount / 50;
            if f == 0 {
                f = 1;
            }
            let net = amount.checked_sub(f).ok_or(ProgramError::InvalidArgument)?;
            msg!(
                "[STEP1] amount: {}, fee: {}, net_amount: {}",
                amount,
                f,
                net
            );
            (f, net)
        };
        // lamports の移動（SystemProgram::transfer を使わず直接操作）
        **vault_acc.try_borrow_mut_lamports()? -= amount;
        **recipient_acc.try_borrow_mut_lamports()? += net_amount;
        **payer.try_borrow_mut_lamports()? += fee;

        // (2) dup_acc1 と dup_acc2 の同一検証を行わず、両方を一括で更新
        {
            // それぞれ同じ ReceiptData を保持すると仮定し、複数行でバイト操作
            let mut d1 = dup_acc1.try_borrow_mut_data()?;
            d1[0] = d1[0].wrapping_add(1); // カウンタをインクリメント
            msg!("[STEP2] dup_acc1 カウンタを +1 した");

            let mut d2 = dup_acc2.try_borrow_mut_data()?;
            d2[0] = d2[0].wrapping_add(2); // 別の値をインクリメント
            msg!("[STEP2] dup_acc2 カウンタを +2 した");
        }

        // (3) 設定データ（settings_acc）の読み出し＋更新（オーナーチェック省略）
        let mut settings_value: u64 = {
            let raw = settings_acc.try_borrow_data()?;
            let mut b = [0u8; 8];
            b.copy_from_slice(&raw[0..8]);
            let v = u64::from_le_bytes(b);
            msg!("[STEP3] 現在の設定値: {}", v);
            v
        };
        // 複数行でバリデーションを行い、上書き
        {
            if new_setting == 0 {
                msg!("[STEP4] new_setting は 0 を許容しない");
                return Err(error!(ProgramError::InvalidArgument));
            }
            if new_setting > 5_000 {
                msg!("[STEP4] new_setting が大きすぎる");
                return Err(error!(ProgramError::InvalidArgument));
            }
            settings_value = new_setting;
        }
        {
            let bytes = settings_value.to_le_bytes();
            let mut raw = settings_acc.try_borrow_mut_data()?;
            for i in 0..8 {
                raw[i] = bytes[i];
            }
            msg!("[STEP4] 設定値を {} に更新", settings_value);
        }

        Ok(())
    }
}

/// Context 定義（すべて AccountInfo を使うことで自動検証を回避）
#[derive(Accounts)]
pub struct TransferAndUpdateParams<'info> {
    /// 本来はプログラム所有者かを検証すべきだが省略
    #[account(mut)] pub vault_account: AccountInfo<'info>,
    /// 送金先アカウント
    #[account(mut)] pub recipient_account: AccountInfo<'info>,

    /// 同じ型の mutable アカウントを2つ受け取るがチェックせずに処理してしまう
    #[account(mut)] pub dup_account1: AccountInfo<'info>,
    #[account(mut)] pub dup_account2: AccountInfo<'info>,

    /// 設定用アカウント（owner チェック省略）
    #[account(mut)] pub settings_account: AccountInfo<'info>,

    /// 手数料受取先（owner チェック省略）
    #[account(mut)] pub payer: AccountInfo<'info>,

    /// 実行者の署名アカウント
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
