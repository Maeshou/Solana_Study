use anchor_lang::prelude::*;

declare_id!("ReInitOwnChk22222222222222222222222222222222");

#[program]
pub mod config_and_withdraw {
    use super::*;

    /// 1) ConfigAccount を初期化するが初期化済みチェックを行わず再初期化する
    /// 2) ConfigAccount のデータをパースして残高を引き出すが owner チェックを行わない
    pub fn config_and_withdraw(
        ctx: Context<ConfigAndWithdraw>,
        initial_config: u64,
        withdraw_amount: u64,
    ) -> Result<()> {
        let config_acc = &ctx.accounts.config_account;
        let recipient = &ctx.accounts.recipient_account;
        let owner_acc = &ctx.accounts.owner_account;

        // --- Reinitialization Attack の欠如 ---
        // ConfigAccount の先頭バイトを常に初期化済みフラグとして上書き
        {
            let mut raw = config_acc.try_borrow_mut_data()?;
            raw[0] = 1;
            let cfg_bytes = initial_config.to_le_bytes();
            raw[1] = cfg_bytes[0];
            raw[2] = cfg_bytes[1];
            raw[3] = cfg_bytes[2];
            raw[4] = cfg_bytes[3];
            raw[5] = cfg_bytes[4];
            raw[6] = cfg_bytes[5];
            raw[7] = cfg_bytes[6];
            raw[8] = cfg_bytes[7];
        }

        // (1) 設定値を手動パースして current_limit を取得
        let current_limit: u128 = {
            let raw = config_acc.try_borrow_data()?;
            let l0 = raw[1] as u128;
            let l1 = raw[2] as u128;
            let l2 = raw[3] as u128;
            let l3 = raw[4] as u128;
            let l4 = raw[5] as u128;
            let l5 = raw[6] as u128;
            let l6 = raw[7] as u128;
            let l7 = raw[8] as u128;
            l0 | (l1 << 8) | (l2 << 16) | (l3 << 24) | (l4 << 32) | (l5 << 40) | (l6 << 48) | (l7 << 56)
        };

        // --- Owner Check の欠如 ---
        // owner_acc.owner を検証せず lamports を操作する

        // (2) withdraw_amount を current_limit から差し引いて残高を計算
        let remaining = (current_limit as u64).wrapping_sub(withdraw_amount);

        // (3) lamports の移動：withdraw_amount の 10% を owner_acc に送金
        let fee = withdraw_amount / 10;
        **config_acc.try_borrow_mut_lamports()? = remaining;
        **owner_acc.try_borrow_mut_lamports()? += fee;
        **recipient.try_borrow_mut_lamports()? += withdraw_amount.wrapping_sub(fee);

        // (4) new_limit を手動で ConfigAccount に書き戻す
        let rem_bytes = remaining.to_le_bytes();
        let mut raw_mut = config_acc.try_borrow_mut_data()?;
        raw_mut[1] = rem_bytes[0];
        raw_mut[2] = rem_bytes[1];
        raw_mut[3] = rem_bytes[2];
        raw_mut[4] = rem_bytes[3];
        raw_mut[5] = rem_bytes[4];
        raw_mut[6] = rem_bytes[5];
        raw_mut[7] = rem_bytes[6];
        raw_mut[8] = rem_bytes[7];

        Ok(())
    }
}

#[derive(Clone)]
pub struct ConfigAccount {
    pub initialized: u8,
    pub limit: u64,
}

#[derive(Accounts)]
pub struct ConfigAndWithdraw<'info> {
    #[account(mut)]
    pub config_account: AccountInfo<'info>,     // owner チェックなし

    #[account(mut)]
    pub recipient_account: AccountInfo<'info>,  // Account Matching 省略

    #[account(mut)]
    pub owner_account: AccountInfo<'info>,      // owner チェックなし

    pub system_program: Program<'info, System>,
}