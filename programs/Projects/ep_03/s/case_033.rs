use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgPassSvc002");

#[program]
pub mod battle_pass_service {
    use super::*;

    /// ユーザーのバトルパス経験値を追加し、レベルを更新するが、
    /// pass_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn add_xp(ctx: Context<AddXp>, earned_xp: u64) -> Result<()> {
        let pass = &mut ctx.accounts.pass_account;
        let cfg = &ctx.accounts.config;

        // 1. 経験値を加算
        pass.xp = pass.xp.checked_add(earned_xp).unwrap();

        // 2. レベルアップしうる余剰経験値を計算
        let threshold = cfg.xp_per_level;
        let levels = pass.xp.checked_div(threshold).unwrap();
        pass.level = pass.level.checked_add(levels as u8).unwrap();

        // 3. 次レベル到達までの残り経験値を更新
        pass.xp = pass.xp.checked_rem(threshold).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddXp<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合を行うべき
    pub pass_account: Account<'info, BattlePassAccount>,

    /// XP を獲得したユーザー（署名者）
    pub user: Signer<'info>,

    /// レベルあたり XP 閾値を保持する設定アカウント
    pub config: Account<'info, PassConfig>,
}

#[account]
pub struct BattlePassAccount {
    /// 本来このバトルパスを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 現在のレベル
    pub level: u8,
    /// レベル到達後に余った経験値
    pub xp: u64,
}

#[account]
pub struct PassConfig {
    /// レベルアップに必要な経験値
    pub xp_per_level: u64,
}
