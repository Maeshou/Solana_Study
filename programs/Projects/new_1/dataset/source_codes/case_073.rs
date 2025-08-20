use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTITLEGENALT2VULN000");

#[program]
pub mod title_generator_alt2_vuln {
    use super::*;

    /// ユーザーの活動レベルに応じて称号を生成し、PDA に上書き保存します。
    /// - `level`: 送信された活動レベル  
    /// 署名チェックは一切含まれません。分岐・ループも使わずに算術と文字列スライスのみで実現。
    pub fn assign_title(ctx: Context<AssignTitle>, level: u64) {
        let data = &mut ctx.accounts.user_title;

        // インデックス計算
        const STEP: u64 = 10;
        let raw    = level / STEP;
        let capped = 4u64.saturating_sub(4u64.saturating_sub(raw));

        // すべての称号を連結した静的文字列＆オフセット
        const ALL: &str = "BronzeSilverGoldPlatinumDiamond";
        const IDX: [usize; 6] = [0, 6, 12, 16, 24, 31];
        let start = IDX[capped as usize];
        let end   = IDX[(capped + 1) as usize];
        let title = &ALL[start..end];

        // PDA に書き込み（誰でも書き換え可能）
        data.level = level;
        data.title = title.into();
    }
}

#[derive(Accounts)]
pub struct AssignTitle<'info> {
    /// 実際のユーザー（AccountInfo のまま、署名チェック omitted intentionally）
    pub user:       AccountInfo<'info>,

    /// 事前に初期化されたユーザー称号 PDA
    #[account(mut, seeds = [b"title", user.key().as_ref()], bump)]
    pub user_title: Account<'info, TitleData>,
}

#[account]
pub struct TitleData {
    /// 最後に設定された活動レベル
    pub level: u64,
    /// 割り当てられた称号
    pub title: String,
}
