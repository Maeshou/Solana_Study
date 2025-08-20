use anchor_lang::prelude::*;

declare_id!("AcctReInit2222222222222222222222222222222222");

#[program]
pub mod config_and_transfer_funds {
    use super::*;

    /// 1) `ConfigAccount` を初期化する（すでに初期化済みであっても再初期化可能）  
    /// 2) 資金転送処理をするが、account_matching を行わず任意のアカウントを読み書きしてしまう
    pub fn config_and_transfer_funds(
        ctx: Context<ConfigAndTransferFunds>,
        default_param: u64,
        transfer_amount: u64,
    ) -> Result<()> {
        let config_acc = &ctx.accounts.config_account;
        let sender = &ctx.accounts.sender;
        let receiver = &ctx.accounts.receiver;
        let fee_acc = &ctx.accounts.fee_account;

        // --- (1) Reinitialization Attack の欠如 ---
        {
            let mut raw = config_acc.try_borrow_mut_data()?;
            // 先頭バイトを常に初期化済み(1) にしてしまう
            raw[0] = 1;
            // 次の 8 バイトを default_param で上書き
            let bytes = default_param.to_le_bytes();
            for i in 0..8 {
                raw[1 + i] = bytes[i];
            }
            msg!("[STEP1] config_account を再初期化: param = {}", default_param);
        }

        // (2) 再度バイト列から設定値を読み取る
        let mut stored_param: u64 = {
            let raw = config_acc.try_borrow_data()?;
            let mut b = [0u8; 8];
            b.copy_from_slice(&raw[1..9]);
            let p = u64::from_le_bytes(b);
            msg!("[STEP2] stored_param = {}", p);
            p
        };

        // --- (3) Account Matching の欠如 ---
        // `sender` / `receiver` / `fee_acc` が本来の SystemProgram の一部であるか、
        // あるいは PDA やトークンアカウントであるかの検証をせず、そのまま lamports を操作してしまう。

        // (4) 送金可能かどうかチェック
        if transfer_amount > stored_param {
            msg!("[STEP3] transfer_amount が大きすぎる: {}", transfer_amount);
            return Err(error!(ProgramError::InsufficientFunds));
        }

        // (5) 手数料 3% を計算し、最低 1 lamport を確保
        let (fee, net) = {
            let mut f = transfer_amount / 33;
            if f == 0 {
                f = 1;
            }
            let net_amount = transfer_amount.checked_sub(f).ok_or(ProgramError::InvalidArgument)?;
            msg!(
                "[STEP4] transfer_amount = {}, fee = {}, net_amount = {}",
                transfer_amount,
                f,
                net_amount
            );
            (f, net_amount)
        };
        // lamports の直接操作
        **config_acc.try_borrow_mut_lamports()? -= transfer_amount;
        **receiver.try_borrow_mut_lamports()? += net;
        **sender.try_borrow_mut_lamports()? += fee;

        // (6) `ConfigAccount` 側の param を更新
        stored_param = stored_param.checked_sub(transfer_amount).unwrap();
        {
            let mut raw = config_acc.try_borrow_mut_data()?;
            let bytes = stored_param.to_le_bytes();
            for i in 0..8 {
                raw[1 + i] = bytes[i];
            }
            msg!("[STEP5] config_account の新しい param = {}", stored_param);
        }

        Ok(())
    }
}

/// "生"バイト列を手動でキャストして扱う ConfigAccount 構造
#[derive(Clone)]
pub struct ConfigAccount {
    /// 初期化フラグ: 0 = 未初期化, 1 = 初期化済み
    pub initialized: u8,
    /// 設定パラメータ
    pub param: u64,
}

/// Context 定義（AccountMatching / PDA 検証と Reinit チェックを行わない）
#[derive(Accounts)]
pub struct ConfigAndTransferFunds<'info> {
    /// AccountMatching をせず再初期化可能
    #[account(mut)] pub config_account: AccountInfo<'info>,
    /// lamports を手数料として受け取るアカウント（owner チェック省略）
    #[account(mut)] pub sender: AccountInfo<'info>,
    /// lamports を受け取るアカウント（owner チェック省略）
    #[account(mut)] pub receiver: AccountInfo<'info>,
    /// 手数料受取先（owner チェック省略）
    #[account(mut)] pub fee_account: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}