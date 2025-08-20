use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfVesting000");

#[program]
pub mod token_vesting {
    use super::*;

    /// ユーザーは現在時刻 `now_ts` に応じた権利確定済みトークン量を受け取ります。
    /// 分岐やループを使わず、算術メソッドとミニマム／サチュレート演算のみで安全に計算します。
    pub fn claim(ctx: Context<Claim>, now_ts: i64) -> Result<()> {
        let rec = &mut ctx.accounts.vesting_record;

        // 経過秒数 = now_ts - start_time （負にならないよう saturating_sub）
        let elapsed = now_ts.saturating_sub(rec.start_time);

        // 経過秒数を u64 にキャストし、duration との最小値を取得
        let elapsed_u = (elapsed as u64).min(rec.duration);

        // 権利確定量 = total_amount * elapsed_u / duration （overflowなし）
        let vested = rec
            .total_amount
            .saturating_mul(elapsed_u)
            .checked_div(rec.duration)
            .unwrap_or(0);

        // これまでに請求済みの量との差分だけ増分更新（saturating_sub で安全に）
        let to_pay = vested.saturating_sub(rec.claimed_amount);
        rec.claimed_amount = rec.claimed_amount.saturating_add(to_pay);

        // 最後に請求したタイムスタンプを更新
        rec.last_claim_ts = now_ts;

        // ログ出力
        msg!(
            "Claimed {} tokens (total vested {}).",
            to_pay,
            rec.claimed_amount
        );
        Ok(())
    }

    /// 現在のレコードをログで確認します。
    pub fn view(ctx: Context<View>) -> Result<()> {
        let rec = &ctx.accounts.vesting_record;
        msg!("User             : {:?}", rec.user);
        msg!("Total Amount     : {}", rec.total_amount);
        msg!("Claimed Amount   : {}", rec.claimed_amount);
        msg!("Start Time (UTC) : {}", rec.start_time);
        msg!("Duration (secs)  : {}", rec.duration);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Claim<'info> {
    /// PDA で再初期化攻撃を防止
    #[account(
        seeds = [b"vesting", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub vesting_record: Account<'info, VestingRecord>,

    /// 請求操作に署名が必要
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct View<'info> {
    /// 誰でも自分のレコードをビュー可能
    #[account(
        seeds = [b"vesting", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub vesting_record: Account<'info, VestingRecord>,

    pub user: Signer<'info>,
}

#[account]
pub struct VestingRecord {
    /// オーナー
    pub user: Pubkey,
    /// 総付与予定量
    pub total_amount: u64,
    /// 既に請求した量
    pub claimed_amount: u64,
    /// 権利確定開始時刻（UTC UNIX epoch）
    pub start_time: i64,
    /// 権利確定に要する期間（秒）
    pub duration: u64,
    /// 最後に請求した時刻（UTC UNIX epoch）
    pub last_claim_ts: i64,
}
