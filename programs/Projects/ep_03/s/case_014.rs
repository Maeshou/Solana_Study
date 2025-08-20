use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgBattleSvc02");

#[program]
pub mod battle_service {
    use super::*;

    /// バトル結果を確定し、ポイントを計算するが、
    /// BattleResult.owner と呼び出しユーザーの照合チェックがない
    pub fn finalize_battle(
        ctx: Context<FinalizeBattle>,
        damage_to_attacker: u64,
        damage_to_defender: u64,
    ) -> Result<()> {
        let res = &mut ctx.accounts.battle_result;

        // 1. 参加者情報を記録
        res.attacker = ctx.accounts.attacker.key();
        res.defender = ctx.accounts.defender.key();

        // 2. 残りHPを計算（デフォルトHP − 受けたダメージ）
        res.attacker_hp = ctx.accounts.config.default_hp
            .checked_sub(damage_to_attacker)
            .unwrap();
        res.defender_hp = ctx.accounts.config.default_hp
            .checked_sub(damage_to_defender)
            .unwrap();

        // 3. ポイントを算出（デフォルトポイント + 残りHP）
        res.attacker_points = ctx.accounts.config.base_points
            .checked_add(res.attacker_hp)
            .unwrap();
        res.defender_points = ctx.accounts.config.base_points
            .checked_add(res.defender_hp)
            .unwrap();

        // 4. 結果反映フラグを更新
        res.resolved = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct FinalizeBattle<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] などで照合を行うべき
    pub battle_result: Account<'info, BattleResult>,

    /// 攻撃側ユーザー（署名者）
    pub attacker: Signer<'info>,

    /// 防御側ユーザー
    pub defender: AccountInfo<'info>,

    /// 初期HP・ベースポイントなど設定を保持するアカウント
    pub config: Account<'info, BattleConfig>,
}

#[account]
pub struct BattleResult {
    /// 本来はこの結果を所有するユーザーの Pubkey
    pub owner: Pubkey,
    pub attacker: Pubkey,
    pub defender: Pubkey,
    pub attacker_hp: u64,
    pub defender_hp: u64,
    pub attacker_points: u64,
    pub defender_points: u64,
    pub resolved: bool,
}

#[account]
pub struct BattleConfig {
    /// 初期HP
    pub default_hp: u64,
    /// ベースポイント
    pub base_points: u64,
}
