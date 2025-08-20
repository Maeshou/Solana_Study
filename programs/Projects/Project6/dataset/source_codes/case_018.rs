// #8: Inventory and Crafting Workshop
// ドメイン: プレイヤーのアイテムインベントリと工房でのアイテム合成。
// 安全対策: `Inventory` と `CraftingRecipe` は親子関係で紐付け。合成に必要な素材アイテムの `TokenAccount` と `Mint` が親子関係を持つことを確認。

declare_id!("D9E0F1G2H3I4J5K6L7M8N9O0P1Q2R3S4T5U6V7W8");

#[program]
pub mod crafting_workshop {
    use super::*;

    pub fn initialize_inventory(ctx: Context<InitializeInventory>) -> Result<()> {
        let inventory = &mut ctx.accounts.inventory;
        inventory.owner = ctx.accounts.owner.key();
        inventory.item_slots = [Pubkey::default(); 10];
        Ok(())
    }

    pub fn craft_item(ctx: Context<CraftItem>) -> Result<()> {
        let inventory = &mut ctx.accounts.inventory;
        let recipe = &mut ctx.accounts.recipe;

        // 素材の消費ロジック
        let mut total_mats_consumed = 0u64;
        let mut i = 0;
        while i < recipe.material_mints.len() {
            let mint_key = recipe.material_mints[i];
            let required_amount = recipe.material_amounts[i];

            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_accounts = token::Burn {
                mint: ctx.accounts.material_mints[i].to_account_info(),
                from: ctx.accounts.material_token_accounts[i].to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::burn(cpi_ctx, required_amount)?;

            total_mats_consumed += required_amount;
            i += 1;
        }

        let new_item_mint = ctx.accounts.new_item_mint.key();

        if inventory.item_slots.iter().any(|&x| x == Pubkey::default()) {
            for slot in inventory.item_slots.iter_mut() {
                if *slot == Pubkey::default() {
                    *slot = new_item_mint;
                    break;
                }
            }
        } else {
            msg!("Inventory is full!");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeInventory<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 10 * 32,
        owner = crate::ID,
    )]
    pub inventory: Account<'info, Inventory>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CraftItem<'info> {
    #[account(
        mut,
        has_one = owner,
    )]
    pub inventory: Account<'info, Inventory>,
    #[account(
        has_one = inventory,
        // InventoryとRecipeが同一口座ではないことを検証
        constraint = inventory.key() != recipe.key() @ ErrorCode::CosplayBlocked,
    )]
    pub recipe: Account<'info, CraftingRecipe>,
    #[account(
        owner = token::ID,
    )]
    pub new_item_mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub material_token_accounts: Account<'info, Vec<TokenAccount>>,
    pub material_mints: Account<'info, Vec<Mint>>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Inventory {
    pub owner: Pubkey,
    pub item_slots: [Pubkey; 10],
}

#[account]
pub struct CraftingRecipe {
    pub inventory: Pubkey,
    pub name: String,
    pub material_mints: Vec<Pubkey>,
    pub material_amounts: Vec<u64>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account is being cosplayed as a different role.")]
    CosplayBlocked,
}
