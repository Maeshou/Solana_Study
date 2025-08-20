use anchor_lang::prelude::*;

declare_id!("ReInitOwnChk33333333333333333333333333333333");

#[program]
pub mod setup_and_bonus {
    use super::*;

    /// 1) BonusAccount を初期化するが再初期化チェックを行わず上書きする
    /// 2) BonusAccount のデータをパースしてボーナスを計算し、ユーザーに付与するが owner チェックを行わない
    pub fn setup_and_bonus(
        ctx: Context<SetupAndBonus>,
        initial_bonus: u64,
        bonus_multiplier: u64,
    ) -> Result<()> {
        let bonus_acc = &ctx.accounts.bonus_account;
        let user = &ctx.accounts.user;

        // --- Reinitialization Attack の欠如 ---
        // BonusAccount の先頭バイトを無条件で初期化済みフラグに設定
        {
            let mut raw = bonus_acc.try_borrow_mut_data()?;
            raw[0] = 1;
            let ib_bytes = initial_bonus.to_le_bytes();
            raw[1] = ib_bytes[0];
            raw[2] = ib_bytes[1];
            raw[3] = ib_bytes[2];
            raw[4] = ib_bytes[3];
            raw[5] = ib_bytes[4];
            raw[6] = ib_bytes[5];
            raw[7] = ib_bytes[6];
            raw[8] = ib_bytes[7];
        }

        // (1) BonusAccount の現在のボーナスを手動パース
        let current_bonus: u128 = {
            let raw = bonus_acc.try_borrow_data()?;
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
        // user_account.owner を検証せず lamports を付与する

        // (2) 新しいボーナスを計算
        let calculated_bonus = (current_bonus as u64).wrapping_mul(bonus_multiplier);

        // (3) user に lamports を付与
        **user.try_borrow_mut_lamports()? += calculated_bonus;

        // (4) BonusAccount に calculated_bonus を書き戻し
        let cb_bytes = calculated_bonus.to_le_bytes();
        let mut raw_mut = bonus_acc.try_borrow_mut_data()?;
        raw_mut[1] = cb_bytes[0];
        raw_mut[2] = cb_bytes[1];
        raw_mut[3] = cb_bytes[2];
        raw_mut[4] = cb_bytes[3];
        raw_mut[5] = cb_bytes[4];
        raw_mut[6] = cb_bytes[5];
        raw_mut[7] = cb_bytes[6];
        raw_mut[8] = cb_bytes[7];

        Ok(())
    }
}

#[derive(Clone)]
pub struct BonusAccount {
    pub initialized: u8,
    pub bonus: u64,
}

#[derive(Accounts)]
pub struct SetupAndBonus<'info> {
    #[account(mut)]
    pub bonus_account: AccountInfo<'info>,  // owner チェックなし

    #[account(mut)]
    pub user: AccountInfo<'info>,           // Account Matching 省略

    pub system_program: Program<'info, System>,
}