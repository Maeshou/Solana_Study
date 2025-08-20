use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRaffleSvc02");

#[program]
pub mod raffle_service {
    use super::*;

    /// ラッフルに参加し、エントリーフィーを支払うが、
    /// has_one = prize_pool のみ検証しており、
    /// raffle_account.owner と ctx.accounts.user.key() の一致照合が抜けているケース
    pub fn enter_raffle(ctx: Context<EnterRaffle>) -> Result<()> {
        let raffle = &mut ctx.accounts.raffle_account;
        let fee = ctx.accounts.config.entry_fee;

        // 1. 参加回数を更新（オーバーフロー保護に saturating_add）
        raffle.entries_count = raffle.entries_count.saturating_add(1);

        // 2. 最後に参加したユーザーを記録
        raffle.last_participant = ctx.accounts.user.key();

        // 3. 直接 Lamports を操作してエントリーフィーを移動
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= fee;
        **ctx.accounts.prize_pool.to_account_info().lamports.borrow_mut() += fee;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnterRaffle<'info> {
    #[account(
        mut,
        has_one = prize_pool,   // プライズプールのアドレスだけ検証
        // 本来は has_one = owner を追加してマッチング検証を行うべき
    )]
    pub raffle_account: Account<'info, RaffleAccount>,

    /// エントリーフィーを貯めるプライズプール
    #[account(mut)]
    pub prize_pool: AccountInfo<'info>,

    /// 参加者のユーザーアカウント（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// ラッフルの設定（エントリーフィーなど）
    pub config: Account<'info, RaffleConfig>,
}

#[account]
pub struct RaffleAccount {
    /// 本来このラッフルを管理するべきオーナーの Pubkey
    pub owner: Pubkey,
    /// ラッフル賞金を保管するプールの Pubkey
    pub prize_pool: Pubkey,
    /// これまでに集まったエントリー数
    pub entries_count: u64,
    /// 最後に参加したユーザーの Pubkey
    pub last_participant: Pubkey,
}

#[account]
pub struct RaffleConfig {
    /// 1 回の参加に必要なエントリーフィー（Lamports）
    pub entry_fee: u64,
}
