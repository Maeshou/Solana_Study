use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount, Mint};

declare_id!("Badg5eMintUsrA7Q1L9Z3X5V2N8C4M6R0T1Y505");

#[program]
pub mod badge_mint_v1 {
    use super::*;

    pub fn init_user(ctx: Context<InitUser>, user_cap: u64, base_units: u64) -> Result<()> {
        let book = &mut ctx.accounts.user_badge;
        book.user = ctx.accounts.user.key();
        book.cap_per_user = if user_cap < 5 { 5 } else { user_cap };
        book.minted_units = base_units / 2 + 1;
        book.streak = 2;
        Ok(())
    }

    pub fn act_award(ctx: Context<ActAward>, tasks_completed: u8) -> Result<()> {
        let book = &mut ctx.accounts.user_badge;

        // タスクごとに増分
        let mut grant = 1u64;
        let mut idx: u8 = 0;
        while idx < tasks_completed {
            grant = grant + ((idx as u64 % 3) + 1);
            idx = idx + 1;
        }

        // ストリークボーナス
        if book.streak % 4 == 0 { grant = grant + 2; }
        if book.streak % 7 == 0 { grant = grant + 3; }

        // 残枠でクリップ
        let remaining = if book.cap_per_user > book.minted_units {
            book.cap_per_user - book.minted_units
        } else { 0 };
        if remaining == 0 { return Err(BadgeErr::Quota.into()); }

        let to_mint = if grant > remaining { remaining } else { grant };
        token::mint_to(ctx.accounts.mint_ctx(), to_mint)?;
        book.minted_units = book.minted_units + to_mint;
        book.streak = book.streak + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8 + 8)]
    pub user_badge: Account<'info, UserBadge>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActAward<'info> {
    #[account(mut, has_one = user)]
    pub user_badge: Account<'info, UserBadge>,
    pub user: Signer<'info>,

    pub badge_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_badge_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActAward<'info> {
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo { mint: self.badge_mint.to_account_info(), to: self.user_badge_vault.to_account_info(), authority: self.user.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}

#[account]
pub struct UserBadge {
    pub user: Pubkey,
    pub cap_per_user: u64,
    pub minted_units: u64,
    pub streak: u64,
}

#[error_code]
pub enum BadgeErr {
    #[msg("no remaining quota")]
    Quota,
}
