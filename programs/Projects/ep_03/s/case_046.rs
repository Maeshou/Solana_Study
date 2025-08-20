use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgAbility001");

#[program]
pub mod ability_service {
    use super::*;

    /// アビリティのクールダウンをリセットするが、
    /// ability_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn reset_cooldown(ctx: Context<ResetCooldown>, ability_id: u64) -> Result<()> {
        let ability = &mut ctx.accounts.ability_account;

        // ↓ 本来は #[account(has_one = owner)] で所有者照合を行うべき
        ability.ability_id = ability_id;
        ability.last_used = 0;                   // 最終使用時刻をリセット
        ability.cooldown = ctx.accounts.config.max_cooldown; // クールダウンを最大値に設定

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ResetCooldown<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合チェックを入れるべき
    pub ability_account: Account<'info, AbilityAccount>,

    /// リセットをリクエストするユーザー（署名者）
    pub user: Signer<'info>,

    /// アビリティの設定（最大クールダウン値）を保持するアカウント
    pub config: Account<'info, AbilityConfig>,
}

#[account]
pub struct AbilityAccount {
    /// 本来このアビリティを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// アビリティの一意識別子
    pub ability_id: u64,
    /// 最後に使用したタイムスタンプ（UNIXタイムスタンプ）
    pub last_used: i64,
    /// 残りクールダウン時間（秒）
    pub cooldown: u64,
}

#[account]
pub struct AbilityConfig {
    /// リセット後のクールダウン最大時間（秒）
    pub max_cooldown: u64,
}
