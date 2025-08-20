use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgEquipSrv001");

#[program]
pub mod equipment_service {
    use super::*;

    /// NFT装備を強化するが、装備アカウントとユーザー所有者の照合チェックがない
    pub fn enhance_equipment(ctx: Context<EnhanceEquipment>, level_increase: u8) -> Result<()> {
        let eq_acc = &mut ctx.accounts.equipment_account;
        // ↓ 本来は eq_acc.owner と ctx.accounts.user.key() の一致を検証すべき

        // 1. レベルを増加
        let new_level = eq_acc.level.checked_add(level_increase).unwrap();
        eq_acc.level = new_level;

        // 2. パワーボーナスを計算 (レベル増加 × 定数係数)
        let bonus = (level_increase as u64)
            .checked_mul(ctx.accounts.config.power_coef)
            .unwrap();

        // 3. パワーステータスに加算
        eq_acc.power = eq_acc.power.checked_add(bonus).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnhanceEquipment<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub equipment_account: Account<'info, EquipmentAccount>,
    /// 強化実行者（署名者）
    pub user: Signer<'info>,
    /// 強化パラメータ（パワー係数）を保持するアカウント
    pub config: Account<'info, EquipConfig>,
}

#[account]
pub struct EquipmentAccount {
    /// 本来この装備を所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// 装備のレベル
    pub level: u8,
    /// 装備のパワーステータス
    pub power: u64,
}

#[account]
pub struct EquipConfig {
    /// レベル増加あたりのパワー係数
    pub power_coef: u64,
}
