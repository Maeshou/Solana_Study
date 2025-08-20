use anchor_lang::prelude::*;

declare_id!("AcctReInit3333333333333333333333333333333333");

#[program]
pub mod setup_and_swap {
    use super::*;

    /// 1) `WalletAccount` を初期化する（何度でも再初期化可能）  
    /// 2) トークンスワップ処理を行うが、account_matching を行わず任意のアカウントを読み書きしてしまう
    pub fn setup_and_swap(
        ctx: Context<SetupAndSwap>,
        initial_coins: u64,
        swap_amount: u64,
    ) -> Result<()> {
        let wallet_acc = &ctx.accounts.wallet_account;
        let token_acc = &ctx.accounts.token_account;
        let fee_acc = &ctx.accounts.fee_account;

        // --- (1) Reinitialization Attack の欠如 ---
        {
            let mut raw = wallet_acc.try_borrow_mut_data()?;
            // 初期化フラグを常に「初期化済み(1)」にセット
            raw[0] = 1;
            // 次の 8 バイトを initial_coins で上書き
            let bytes = initial_coins.to_le_bytes();
            for i in 0..8 {
                raw[1 + i] = bytes[i];
            }
            msg!("[STEP1] wallet_account を再初期化: coins = {}", initial_coins);
        }

        // (2) 再度残高を読み取る
        let mut stored_coins: u64 = {
            let raw = wallet_acc.try_borrow_data()?;
            let mut b = [0u8; 8];
            b.copy_from_slice(&raw[1..9]);
            let c = u64::from_le_bytes(b);
            msg!("[STEP2] stored_coins = {}", c);
            c
        };

        // --- (3) Account Matching の欠如 ---
        // `token_acc` が本当にトークンメタデータ PDA であるかの検証をせず、そのまま lamports を操作してしまう。

        // (4) スワップ可能かチェック
        if swap_amount > stored_coins {
            msg!("[STEP3] swap_amount が大きすぎる: {}", swap_amount);
            return Err(error!(ProgramError::InsufficientFunds));
        }

        // (5) 手数料 4% を計算し、最低 2 lamports を確保
        let (fee, net) = {
            let mut f = swap_amount / 25;
            if f < 2 {
                f = 2;
            }
            let net_amount = swap_amount.checked_sub(f).ok_or(ProgramError::InvalidArgument)?;
            msg!(
                "[STEP4] swap_amount = {}, fee = {}, net_amount = {}",
                swap_amount,
                f,
                net_amount
            );
            (f, net_amount)
        };
        // lamports の直接操作
        **wallet_acc.try_borrow_mut_lamports()? -= swap_amount;
        **token_acc.try_borrow_mut_lamports()? += net;
        **fee_acc.try_borrow_mut_lamports()? += fee;

        // (6) `WalletAccount` 側の残高を更新
        stored_coins = stored_coins.checked_sub(swap_amount).unwrap();
        {
            let mut raw = wallet_acc.try_borrow_mut_data()?;
            let bytes = stored_coins.to_le_bytes();
            for i in 0..8 {
                raw[1 + i] = bytes[i];
            }
            msg!("[STEP5] wallet_account の新しい coins = {}", stored_coins);
        }

        Ok(())
    }
}

/// "生"バイト列を手動でキャストして扱う WalletAccount 構造
#[derive(Clone)]
pub struct WalletAccount {
    /// 初期化フラグ: 0 = 未初期化, 1 = 初期化済み
    pub initialized: u8,
    /// コイン数
    pub coins: u64,
}

/// Context 定義（AccountMatching / PDA 検証と Reinit チェックを行わない）
#[derive(Accounts)]
pub struct SetupAndSwap<'info> {
    /// AccountMatching をせず再初期化可能
    #[account(mut)] pub wallet_account: AccountInfo<'info>,
    /// トークンアカウント（owner / PDA チェックなし）
    #[account(mut)] pub token_account: AccountInfo<'info>,
    /// 手数料受取先（owner チェック省略）
    #[account(mut)] pub fee_account: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}