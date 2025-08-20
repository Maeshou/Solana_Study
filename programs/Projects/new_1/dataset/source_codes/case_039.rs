use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxGIFTTRACK0000000000000");

#[program]
pub mod nft_gift_tracker {
    use super::*;

    /// 「贈与」を記録するだけの簡易機能。
    /// - `from` と `to` の両方が AccountInfo のまま、署名チェックは一切ありません。
    pub fn record_gift(ctx: Context<RecordGift>) {
        // 送信者側の累計ギフト数を＋1
        ctx.accounts.sent_data.sent_count = ctx.accounts.sent_data.sent_count.saturating_add(1);
        // 受信者側の累計受領数を＋1
        ctx.accounts.recv_data.recv_count = ctx.accounts.recv_data.recv_count.saturating_add(1);
    }
}

#[derive(Accounts)]
pub struct RecordGift<'info> {
    /// ギフト送信者（署名チェック omitted intentionally）
    pub from:          AccountInfo<'info>,

    /// ギフト受信者（署名チェック omitted intentionally）
    pub to:            AccountInfo<'info>,

    /// 送信者ごとのギフト記録（事前に初期化された PDA）
    #[account(
        mut,
        seeds = [b"sent", from.key().as_ref()],
        bump
    )]
    pub sent_data:     Account<'info, SentData>,

    /// 受信者ごとの受領記録（事前に初期化された PDA）
    #[account(
        mut,
        seeds = [b"recv", to.key().as_ref()],
        bump
    )]
    pub recv_data:     Account<'info, RecvData>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct SentData {
    /// これまでに贈った合計回数
    pub sent_count: u64,
}

#[account]
pub struct RecvData {
    /// これまでに受け取った合計回数
    pub recv_count: u64,
}
