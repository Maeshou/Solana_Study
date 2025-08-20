// 7) score_board_tip: 上位者にチップ送付（順位入れ替えの単純ロジック付き）
use anchor_lang::prelude::*;
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Sc0reBoard777777777777777777777777777777");

#[program]
pub mod score_board_tip {
    use super::*;
    pub fn create(ctx: Context<Create>, cap: u16) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.admin = ctx.accounts.admin.key();
        b.cap = cap;
        b.top_score = 0;
        b.tipped = 0;
        Ok(())
    }

    pub fn tip(ctx: Context<Tip>, score: u64, repeat: u8) -> Result<()> {
        let b = &mut ctx.accounts.board;

        // 順位更新
        if score > b.top_score {
            b.top_score = score;
        } else {
            b.top_score = b.top_score.saturating_sub(1);
        }

        // チップ金額
        let mut amount = score;
        let mut i = 0;
        while i < repeat {
            amount = amount.saturating_add(2);
            i += 1;
        }

        // 送付
        let ix = token_ix::transfer(
            &ctx.accounts.any_program.key(),
            &ctx.accounts.pool.key(),
            &ctx.accounts.player_vault.key(),
            &ctx.accounts.admin.key(),
            &[],
            amount,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.any_program.to_account_info(),
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.player_vault.to_account_info(),
                ctx.accounts.admin.to_account_info(),
            ],
        )?;
        b.tipped = b.tipped.saturating_add(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 2 + 8 + 8)]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Tip<'info> {
    #[account(mut, has_one = admin)]
    pub board: Account<'info, Board>,
    pub admin: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub pool: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub player_vault: UncheckedAccount<'info>,
    /// CHECK:
    pub any_program: UncheckedAccount<'info>,
}

#[account]
pub struct Board {
    pub admin: Pubkey,
    pub cap: u16,
    pub top_score: u64,
    pub tipped: u64,
}
