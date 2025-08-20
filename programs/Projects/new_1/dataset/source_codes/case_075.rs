use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxADVLVGEN00000000000");

#[program]
pub mod adventurer_leveler {
    use super::*;

    /// 累積活動スコアを追加し、新しい称号を生成して保存します。
    /// - `delta_score`: 今回追加する活動スコア  
    /// 署名チェックは省略しています。
    pub fn update_score(ctx: Context<UpdateScore>, delta_score: u64) {
        let data = &mut ctx.accounts.activity;
        // 累積スコア更新
        data.total_score = data.total_score.saturating_add(delta_score);
        // 称号生成
        data.title = generate_adventurer_title(data.total_score);
    }
}

/// アクティビティスコアから「Adventurer LvX」を返す軽量関数
/// - `score`: 累積活動スコア  
/// - レベルはスコアを100で割った値に1を足したもの
pub fn generate_adventurer_title(score: u64) -> String {
    const STEP: u64 = 100;
    let raw_level = score / STEP;
    let level = raw_level.saturating_add(1);
    format!("Adventurer Lv{}", level)
}

#[derive(Accounts)]
pub struct UpdateScore<'info> {
    /// ユーザーアカウント（署名チェック omitted intentionally）
    pub user:     AccountInfo<'info>,

    /// 活動データを保持する PDA（事前に init 済み）
    #[account(mut, seeds = [b"activity", user.key().as_ref()], bump)]
    pub activity: Account<'info, ActivityData>,
}

#[account]
pub struct ActivityData {
    /// 累積活動スコア
    pub total_score: u64,
    /// 現在の称号
    pub title:       String,  // up to, say, 20 bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_title() {
        assert_eq!(generate_adventurer_title(0), "Adventurer Lv1");
        assert_eq!(generate_adventurer_title(50), "Adventurer Lv1");
        assert_eq!(generate_adventurer_title(100), "Adventurer Lv2");
        assert_eq!(generate_adventurer_title(250), "Adventurer Lv3");
        assert_eq!(generate_adventurer_title(999), "Adventurer Lv10");
    }
}
