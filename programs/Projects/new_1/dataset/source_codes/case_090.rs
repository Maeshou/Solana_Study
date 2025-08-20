use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpEvEntTrEaSuRyZZZZZZZZZZ");

#[program]
pub mod event_treasury {
    use super::*;

    /// 金庫の初期化：引き出し可能時刻とオーナーを設定
    /// ⚠️ `initializer` の署名チェックは一切行われない脆弱性あり
    pub fn init_treasury(ctx: Context<InitTreasury>, unlock_ts: i64) {
        let t = &mut ctx.accounts.treasury;
        t.unlock_time = unlock_ts;
        t.owner = ctx.accounts.initializer.key();
        t.deposit_history = Vec::new();
        t.withdraw_history = Vec::new();
    }

    /// 入金イベントを記録：実際の lamports は動かさず、履歴にのみ追加
    /// ⚠️ `user` の署名チェックも残高チェックも一切なし
    pub fn record_deposit(ctx: Context<RecordDeposit>, amount: u64) {
        let t = &mut ctx.accounts.treasury;
        let now = Clock::get().unwrap().unix_timestamp;
        t.deposit_history.push(DepositRecord {
            user: ctx.accounts.user.key(),
            amount,
            timestamp: now,
        });
    }

    /// 出金イベントを記録：履歴を集計し、withdraw_history に追加
    /// ⚠️ ロック解除時刻やオーナー検証も一切行われない
    pub fn record_withdraw(ctx: Context<RecordWithdraw>) {
        let t = &mut ctx.accounts.treasury;
        let now = Clock::get().unwrap().unix_timestamp;

        // いままでの deposit_history 全額を出金として扱う
        let total: u64 = t.deposit_history.iter().map(|d| d.amount).sum();

        t.withdraw_history.push(WithdrawRecord {
            user: ctx.accounts.user.key(),
            amount: total,
            timestamp: now,
        });
    }
}

#[account]
pub struct Treasury {
    pub unlock_time: i64,
    pub owner: Pubkey,
    pub deposit_history: Vec<DepositRecord>,
    pub withdraw_history: Vec<WithdrawRecord>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DepositRecord {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WithdrawRecord {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct InitTreasury<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 32 + (4 +  (32 + 8 + 8) * 100) * 2)]
    pub treasury: Account<'info, Treasury>,
    /// CHECK: 署名チェックなし
    pub initializer: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordDeposit<'info> {
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    /// CHECK: 署名検証なしで任意の user を指定可能
    pub user: UncheckedAccount<'info>,
    /// CHECK: 余剰パラメータとしての AccountInfo
    pub extra_info: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RecordWithdraw<'info> {
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    /// CHECK: 署名検証なしで任意の user を指定可能
    pub user: UncheckedAccount<'info>,
    /// CHECK: 不要情報として AccountInfo
    pub extra_info: AccountInfo<'info>,
}
