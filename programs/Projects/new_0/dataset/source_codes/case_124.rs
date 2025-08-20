use anchor_lang::prelude::*;

declare_id!("Rcpe111111111111111111111111111111111111");

#[program]
pub mod recipe_manager {
    /// レシピを新規登録
    pub fn add_recipe(
        ctx: Context<AddRecipe>,
        title: String,
        instructions: String,
    ) -> Result<()> {
        // タイトル長チェック
        if title.len() > 64 {
            return Err(ErrorCode::TitleTooLong.into());
        }
        // 作り方長チェック
        if instructions.len() > 1024 {
            return Err(ErrorCode::InstructionsTooLong.into());
        }

        let r = &mut ctx.accounts.recipe;
        r.owner        = ctx.accounts.user.key();
        r.title        = title;
        r.instructions = instructions;
        Ok(())
    }

    /// 登録済みレシピを更新
    pub fn update_recipe(
        ctx: Context<UpdateRecipe>,
        new_title: String,
        new_instructions: String,
    ) -> Result<()> {
        // バリデーション
        if new_title.len() > 64 {
            return Err(ErrorCode::TitleTooLong.into());
        }
        if new_instructions.len() > 1024 {
            return Err(ErrorCode::InstructionsTooLong.into());
        }

        let r = &mut ctx.accounts.recipe;
        // 所有者チェック
        if r.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::Unauthorized.into());
        }

        r.title        = new_title;
        r.instructions = new_instructions;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddRecipe<'info> {
    /// 同一アカウント再初期化防止（Reinit Attack）
    #[account(init, payer = user, space = 8 + 32 + 4 + 64 + 4 + 1024)]
    pub recipe:       Account<'info, RecipeAccount>,

    /// 操作するユーザー（Signer Authorization）
    #[account(mut)]
    pub user:         Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateRecipe<'info> {
    /// 型チェック＆所有者チェック (Owner Check / Type Cosplay)
    #[account(mut)]
    pub recipe:       Account<'info, RecipeAccount>,

    /// 実際に署名したユーザー
    pub user:         Signer<'info>,
}

#[account]
pub struct RecipeAccount {
    /// このレシピを操作できるユーザー
    pub owner:        Pubkey,
    /// レシピタイトル（最大64文字）
    pub title:        String,
    /// 作り方詳細（最大1024文字）
    pub instructions: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("タイトルが長すぎます")]
    TitleTooLong,
    #[msg("作り方が長すぎます")]
    InstructionsTooLong,
}
