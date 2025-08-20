use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTITLEGEN00000000000000");

#[program]
pub mod title_generator {
    use super::*;

    /// ユーザーの活動レベルに応じて称号を生成し、PDA に保存します。
    /// - `level`: 送信された活動レベル
    /// 署名チェックは一切行いません。分岐・ループも使わずに配列＋算術のみで実現。
    pub fn assign_title(ctx: Context<AssignTitle>, level: u64) {
        // 称号一覧（レベル区分ごと）
        const TITLES: [&str; 5] = ["Bronze", "Silver", "Gold", "Platinum", "Diamond"];
        // 1 区分あたりのレベル幅
        const STEP: u64 = 10;
        // 区分インデックスを算出し、配列長-1 未満にキャップ
        let idx = (level.checked_div(STEP).unwrap_or(0) as usize)
            .min(TITLES.len() - 1);
        // 選ばれた称号を文字列化
        let title_str = TITLES[idx];

        // PDA に書き込み
        let data = &mut ctx.accounts.user_title;
        data.level = level;
        data.title = title_str.to_string();
    }
}

#[derive(Accounts)]
pub struct AssignTitle<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:  Signer<'info>,

    /// 実際のユーザー（署名チェック omitted intentionally）
    pub user:       AccountInfo<'info>,

    /// ユーザーごとの称号データ PDA
    #[account(
        init_if_needed,
        payer    = fee_payer,
        seeds    = [b"title", user.key().as_ref()],
        bump,
        space    = 8  /* discriminator */
                 + 8  /* level */
                 + (4 + 32) /* title String (max 32 bytes) */
    )]
    pub user_title: Account<'info, TitleData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct TitleData {
    /// 最後に送信された活動レベル
    pub level: u64,
    /// 割り当てられた称号
    pub title: String,
}
