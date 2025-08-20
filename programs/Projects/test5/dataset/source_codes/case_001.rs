use anchor_lang::prelude::*;

declare_id!("ReInitOwnChk11111111111111111111111111111111");

#[program]
pub mod initialize_and_deposit {
    use super::*;

    /// 1) DataAccount を初期化するが、初期化済みかのチェックを行わず再初期化する
    /// 2) ユーザーから lamports を預け入れるが owner チェックを行わない
    pub fn initialize_and_deposit(
        ctx: Context<InitializeAndDeposit>,
        initial_amount: u64,
        deposit_amount: u64,
    ) -> Result<()> {
        let data_acc = &ctx.accounts.data_account;
        let user = &ctx.accounts.user;
        let fee_acc = &ctx.accounts.fee_account;

        // --- Reinitialization Attack の欠如 ---
        // DataAccount の先頭バイトを初期化済フラグとして常に書き込む
        {
            let mut raw = data_acc.try_borrow_mut_data()?;
            raw[0] = 1; // 初期化済みフラグ
            // 次の 8 バイトを initial_amount に上書き
            let bytes = initial_amount.to_le_bytes();
            raw[1] = bytes[0];
            raw[2] = bytes[1];
            raw[3] = bytes[2];
            raw[4] = bytes[3];
            raw[5] = bytes[4];
            raw[6] = bytes[5];
            raw[7] = bytes[6];
            raw[8] = bytes[7];
        }

        // (1) 設定された初期残高を読み取る（手動パース）
        let mut current_balance: u128 = {
            let raw = data_acc.try_borrow_data()?;
            let b0 = raw[1] as u128;
            let b1 = raw[2] as u128;
            let b2 = raw[3] as u128;
            let b3 = raw[4] as u128;
            let b4 = raw[5] as u128;
            let b5 = raw[6] as u128;
            let b6 = raw[7] as u128;
            let b7 = raw[8] as u128;
            b0 | (b1 << 8) | (b2 << 16) | (b3 << 24) | (b4 << 32) | (b5 << 40) | (b6 << 48) | (b7 << 56)
        };

        // --- Owner Check の欠如 ---
        // fee_acc.owner を検証せず手数料を送金する

        // (2) deposit_amount を current_balance に加算
        let added_balance = current_balance + (deposit_amount as u128);

        // (3) lamports の移動：deposit_amount の 5% を fee_acc に送金
        let fee = deposit_amount / 20; // 5% fee
        **data_acc.try_borrow_mut_lamports()? = (current_balance as u64).wrapping_add(deposit_amount).wrapping_sub(fee);
        **fee_acc.try_borrow_mut_lamports()? += fee;
        **user.try_borrow_mut_lamports()? -= deposit_amount;

        // (4) new_balance を DataAccount に書き戻す（手動書き込み）
        let new_bytes = (added_balance as u64).to_le_bytes();
        let mut raw_mut = data_acc.try_borrow_mut_data()?;
        raw_mut[1] = new_bytes[0];
        raw_mut[2] = new_bytes[1];
        raw_mut[3] = new_bytes[2];
        raw_mut[4] = new_bytes[3];
        raw_mut[5] = new_bytes[4];
        raw_mut[6] = new_bytes[5];
        raw_mut[7] = new_bytes[6];
        raw_mut[8] = new_bytes[7];

        Ok(())
    }
}

#[derive(Clone)]
pub struct DataAccount {
    pub initialized: u8,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct InitializeAndDeposit<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // owner チェックなし

    #[account(mut)]
    pub user: AccountInfo<'info>,          // Account Matching 省略

    #[account(mut)]
    pub fee_account: AccountInfo<'info>,   // owner チェックなし

    pub system_program: Program<'info, System>,
}