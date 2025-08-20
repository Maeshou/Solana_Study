// #2: Equipment Enhancement
// ドメイン: 装備アイテムのレベルアップと強化。素材アイテムの消費。
// 安全対策: 強化する装備と素材アイテムが異なるMintをもち、`constraint` で Mint の不一致を強制。また、強化ステータスは `Equipment` 口座に紐づき、所有者検証と親子関係で安全を確保。

declare_id!("H6G7F8E9D0C1B2A3Z4Y5X6W7V8U9T0S1R2Q3P4O5");

#[program]
pub mod equip_enhancer {
    use super::*;

    pub fn initialize_equipment(ctx: Context<InitializeEquipment>, item_id: u32) -> Result<()> {
        let equipment = &mut ctx.accounts.equipment;
        equipment.owner = ctx.accounts.owner.key();
        equipment.item_id = item_id;
        equipment.level = 0;
        equipment.enhancement_level = 0;
        equipment.is_active = true;
        equipment.stat_boosts = [0; 4];
        Ok(())
    }

    pub fn enhance_equipment(ctx: Context<EnhanceEquipment>) -> Result<()> {
        let equipment = &mut ctx.accounts.equipment;
        let material = &mut ctx.accounts.material_token_account;

        if material.amount < 1 {
            return err!(ErrorCode::NotEnoughMaterials);
        }

        // CPIでトークンを燃やす
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = token::Burn {
            mint: ctx.accounts.material_mint.to_account_info(),
            from: ctx.accounts.material_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::burn(cpi_ctx, 1)?;

        // 強化ロジック
        let mut new_level = equipment.enhancement_level.checked_add(1).unwrap();
        equipment.enhancement_level = new_level;

        // ステータスブーストを更新
        let stat_boost = new_level.checked_mul(2).unwrap_or(u32::MAX);
        equipment.stat_boosts[0] = stat_boost;
        equipment.stat_boosts[1] = stat_boost.wrapping_add(1);

        for i in 0..4 {
            equipment.stat_boosts[i] = equipment.stat_boosts[i].checked_add(new_level.into()).unwrap_or(u32::MAX);
        }

        // 簡易ニュートン法的な処理
        let base_stat = 100u64;
        let mut x = base_stat;
        for _ in 0..5 {
            x = (x + base_stat.checked_div(x).unwrap_or(1)) / 2;
        }

        if equipment.enhancement_level % 5 == 0 {
            msg!("Major enhancement achieved!");
        } else {
            msg!("Minor enhancement.");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeEquipment<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4 + 4 + 1 + 4 * 4,
        owner = crate::ID,
    )]
    pub equipment: Account<'info, Equipment>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnhanceEquipment<'info> {
    #[account(
        mut,
        has_one = owner,
    )]
    pub equipment: Account<'info, Equipment>,
    #[account(
        mut,
        // TokenAccountとMintの親子関係を強制
        constraint = material_token_account.mint == material_mint.key() @ ErrorCode::MintMismatch,
        // 強化される装備と素材のMintが異なることを強制
        constraint = equipment_mint.key() != material_mint.key() @ ErrorCode::CosplayBlocked,
    )]
    pub material_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        owner = token::ID,
    )]
    pub material_mint: Account<'info, Mint>,
    #[account(
        owner = token::ID,
    )]
    pub equipment_mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Equipment {
    pub owner: Pubkey,
    pub item_id: u32,
    pub level: u32,
    pub enhancement_level: u32,
    pub is_active: bool,
    pub stat_boosts: [u32; 4],
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account is being cosplayed as a different role.")]
    CosplayBlocked,
    #[msg("Material mint does not match the token account's mint.")]
    MintMismatch,
    #[msg("Not enough materials to enhance.")]
    NotEnoughMaterials,
}
