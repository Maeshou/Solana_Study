use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUN");

#[program]
pub mod recipe_manager {
    use super::*;

    /// レシピ作成：ID と名前・説明を受け取り、最小限のフィールドを設定
    pub fn initialize_recipe(
        ctx: Context<InitializeRecipe>,
        recipe_id: u64,
        name: String,
        description: String,
    ) -> Result<()> {
        let recipe = &mut ctx.accounts.recipe;
        // アカウントはゼロクリア済 → 必要なフィールドだけ代入
        recipe.owner           = ctx.accounts.user.key();
        recipe.bump            = *ctx.bumps.get("recipe").unwrap();
        recipe.recipe_id       = recipe_id;
        recipe.name            = name;
        recipe.description     = description;
        let now = ctx.accounts.clock.unix_timestamp;
        recipe.times_cooked    = 0;
        recipe.total_rating    = 0;
        recipe.rating_count    = 0;
        recipe.average_rating  = 0;
        recipe.archived        = false;
        recipe.last_updated_ts = now;
        Ok(())
    }

    /// 調理回数登録：`times_cooked` をインクリメントし、タイムスタンプを更新
    pub fn cook_recipe(
        ctx: Context<ModifyRecipe>,
    ) -> Result<()> {
        let r = &mut ctx.accounts.recipe;
        r.times_cooked    = r.times_cooked.wrapping_add(1);
        r.last_updated_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// 評価投稿：合計評価・件数を更新し、平均を再計算、タイムスタンプも更新
    pub fn submit_rating(
        ctx: Context<ModifyRecipe>,
        rating: u8,
    ) -> Result<()> {
        let r = &mut ctx.accounts.recipe;
        r.total_rating    = r.total_rating.wrapping_add(rating as u64);
        r.rating_count    = r.rating_count.wrapping_add(1);
        r.average_rating  = (r.total_rating / r.rating_count) as u8;
        r.last_updated_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// レシピ名変更：`name` を更新し、タイムスタンプを更新
    pub fn rename_recipe(
        ctx: Context<ModifyRecipe>,
        new_name: String,
    ) -> Result<()> {
        let r = &mut ctx.accounts.recipe;
        r.name            = new_name;
        r.last_updated_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// アーカイブ切り替え：`archived` をトグルし、タイムスタンプを更新
    pub fn toggle_archive(
        ctx: Context<ModifyRecipe>,
    ) -> Result<()> {
        let r = &mut ctx.accounts.recipe;
        r.archived        = !r.archived;
        r.last_updated_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(recipe_id: u64)]
pub struct InitializeRecipe<'info> {
    /// ゼロクリア済み → 必要フィールドだけ設定
    #[account(
        init_zeroed,
        payer = user,
        seeds = [b"recipe", user.key().as_ref(), &recipe_id.to_le_bytes()],
        bump,
        space = 8 + 32 + 1 + 8  // discriminator + owner + bump + recipe_id
              + 4 + 100       // name（最大100バイト）
              + 4 + 200       // description（最大200バイト）
              + 8             // times_cooked
              + 8             // total_rating
              + 8             // rating_count
              + 1             // average_rating
              + 1             // archived
              + 8             // last_updated_ts
    )]
    pub recipe: Account<'info, Recipe>,

    /// レシピ作成者（署名必須）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyRecipe<'info> {
    /// 既存の Recipe（PDA 検証 + オーナーチェック）
    #[account(
        mut,
        seeds = [b"recipe", owner.key().as_ref(), &recipe.recipe_id.to_le_bytes()],
        bump = recipe.bump,
        has_one = owner
    )]
    pub recipe: Account<'info, Recipe>,

    /// レシピ所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Recipe {
    pub owner:           Pubkey,
    pub bump:            u8,
    pub recipe_id:       u64,
    pub name:            String,
    pub description:     String,
    pub times_cooked:    u64,
    pub total_rating:    u64,
    pub rating_count:    u64,
    pub average_rating:  u8,
    pub archived:        bool,
    pub last_updated_ts: i64,
}
