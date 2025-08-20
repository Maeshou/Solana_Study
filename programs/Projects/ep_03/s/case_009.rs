use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgPetCare001");

#[program]
pub mod pet_care_service {
    use super::*;

    /// フードを消費してペットの体力を回復するが、
    /// pet_account.owner と ctx.accounts.user.key() の一致を検証していない
    pub fn feed_pet(ctx: Context<FeedPet>, food_amount: u8) -> Result<()> {
        let pet = &mut ctx.accounts.pet_account;

        // 回復量を計算（food_amount × 単位回復量）
        let heal_amount = (food_amount as u64)
            .checked_mul(ctx.accounts.config.health_per_food)
            .unwrap();

        // 体力と消費フード数を更新
        pet.health = pet.health.checked_add(heal_amount).unwrap();
        pet.food_consumed = pet.food_consumed.checked_add(food_amount as u64).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct FeedPet<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合すべき
    pub pet_account: Account<'info, PetAccount>,

    /// フードを使うユーザー（署名者）
    pub user: Signer<'info>,

    /// 回復量設定を保持するアカウント
    pub config: Account<'info, PetConfig>,
}

#[account]
pub struct PetAccount {
    /// このペットを所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// 現在の体力
    pub health: u64,
    /// これまでに消費したフードの総数
    pub food_consumed: u64,
}

#[account]
pub struct PetConfig {
    /// フード1個あたりの回復量
    pub health_per_food: u64,
}
