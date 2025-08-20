use anchor_lang::prelude::*;

declare_id!("AcctReInit1111111111111111111111111111111111");

#[program]
pub mod initialize_and_process_payment {
    use super::*;

    /// 1) `DataAccount` を初期化する（すでに初期化済みであっても再初期化可能）  
    /// 2) 支払い処理を行うが、account_matching を行わず任意のアカウントを読み書きしてしまう
    pub fn initialize_and_process_payment(
        ctx: Context<InitializeAndProcessPayment>,
        initial_balance: u64,
        pay_amount: u64,
    ) -> Result<()> {
        let data_acc = &ctx.accounts.data_account;
        let payer = &ctx.accounts.payer;
        let payee = &ctx.accounts.payee;
        let system_program = &ctx.accounts.system_program;

        // --- (1) Reinitialization Attack の欠如 ---
        // data_acc がすでに初期化済みかどうかをチェックせず、以下のコードで何度でも再初期化してしまう。
        {
            let mut raw = data_acc.try_borrow_mut_data()?;
            // 先頭バイトを初期化フラグとし、無条件で「初期化済み(1)」に変更
            raw[0] = 1;
            // 次の 8 バイトを initial_balance に上書き
            let bytes = initial_balance.to_le_bytes();
            for i in 0..8 {
                raw[1 + i] = bytes[i];
            }
            msg!("[STEP1] data_account を再初期化: balance = {}", initial_balance);
        }

        // (2) 再度バイト列から残高を読み取る（複数行で手動デシリアライズ）
        let mut stored_balance: u64 = {
            let raw = data_acc.try_borrow_data()?;
            let mut b = [0u8; 8];
            b.copy_from_slice(&raw[1..9]);
            let bal = u64::from_le_bytes(b);
            msg!("[STEP2] stored_balance = {}", bal);
            bal
        };

        // --- (3) Account Matching の欠如 ---
        // `payer` / `payee` が本来想定するトークンや PDA のアカウントかを検証せず、そのまま lamports を操作してしまう。

        // (4) 支払い額が足りているかチェック
        if pay_amount > stored_balance {
            msg!("[STEP3] pay_amount が大きすぎる: {}", pay_amount);
            return Err(error!(ProgramError::InsufficientFunds));
        }

        // (5) 手数料計算＋lamports の移動（複数行で多様化）
        let (fee, net) = {
            // 2% の手数料を計算し、最低 1 lamport を確保
            let mut f = pay_amount / 50;
            if f == 0 {
                f = 1;
            }
            let net_amount = pay_amount.checked_sub(f).ok_or(ProgramError::InvalidArgument)?;
            msg!(
                "[STEP4] pay_amount = {}, fee = {}, net_amount = {}",
                pay_amount,
                f,
                net_amount
            );
            (f, net_amount)
        };
        // lamports の直接操作（CPI を使わず）
        **data_acc.try_borrow_mut_lamports()? -= pay_amount;
        **payee.try_borrow_mut_lamports()? += net;
        **payer.try_borrow_mut_lamports()? += fee;

        // (6) `DataAccount` 側の残高を更新
        stored_balance = stored_balance.checked_sub(pay_amount).unwrap();
        {
            let mut raw = data_acc.try_borrow_mut_data()?;
            let bytes = stored_balance.to_le_bytes();
            for i in 0..8 {
                raw[1 + i] = bytes[i];
            }
            msg!("[STEP5] data_account の新しい残高 = {}", stored_balance);
        }

        Ok(())
    }
}

/// "生"バイト列を手動でキャストして扱う DataAccount 構造
#[derive(Clone)]
pub struct DataAccount {
    /// 初期化フラグ: 0 = 未初期化, 1 = 初期化済み
    pub initialized: u8,
    /// 残高フィールド
    pub balance: u64,
}

/// Context 定義（AccountMatching / PDA 検証と Reinit チェックを行わない）
#[derive(Accounts)]
pub struct InitializeAndProcessPayment<'info> {
    /// AccountMatching を行わず何度でも初期化可能
    #[account(mut)] pub data_account: AccountInfo<'info>,
    /// 支払い手数料受取先（owner チェック省略）
    #[account(mut)] pub payer: AccountInfo<'info>,
    /// 実際に受け取るアカウント（owner チェック省略）
    #[account(mut)] pub payee: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}