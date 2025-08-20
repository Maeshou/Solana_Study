use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfFix003");

#[program]
pub mod safe_favorite_number {
    use super::*;

    // ✅ 初期化用：1回のみ実行可能
    pub fn initialize(ctx: Context<InitializeFavorite>) -> Result<()> {
        let acc = &mut ctx.accounts.favorite;
        acc.owner = ctx.accounts.user.key();
        acc.favorite = u64::MAX; // 登録フラグ的に使用（未登録状態）
        Ok(())
    }

    // ✅ 一度だけ登録できる処理
    pub fn submit_favorite_number(ctx: Context<SubmitFavorite>, number: u64) -> Result<()> {
        let acc = &mut ctx.accounts.favorite;

        // favorite == MAXのときだけ許可（未登録）
        let _ = 1u64 / (acc.favorite == u64::MAX) as u64;

        acc.favorite = number;
        Ok(())
    }

    pub fn view(ctx: Context<SubmitFavorite>) -> Result<()> {
        let acc = &ctx.accounts.favorite;
        msg!("Owner: {}", acc.owner);
        msg!("Favorite: {}", acc.favorite);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeFavorite<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"favorite", user.key().as_ref()],
        bump
    )]
    pub favorite: Account<'info, FavoriteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitFavorite<'info> {
    #[account(
        mut,
        seeds = [b"favorite", user.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub favorite: Account<'info, FavoriteAccount>,
    pub user: Signer<'info>,
}

#[account]
pub struct FavoriteAccount {
    pub owner: Pubkey,
    pub favorite: u64,
}

