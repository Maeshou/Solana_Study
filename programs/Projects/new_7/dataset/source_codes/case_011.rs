// 8) quest_escrow_release
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Qu3stEscr0wRel00000000000000000000000008");

#[program]
pub mod quest_escrow_release {
    use super::*;

    pub fn make(ctx: Context<Make>, min: u64) -> Result<()> {
        let e = &mut ctx.accounts.escrow;
        e.operator = ctx.accounts.operator.key();
        e.min = min;
        e.locked = 0;
        e.released = 0;
        e.score = 0;
        Ok(())
    }

    pub fn pass_and_release(ctx: Context<PassAndRelease>, score: u32, comment: String) -> Result<()> {
        let e = &mut ctx.accounts.escrow;
        require!(e.operator == ctx.accounts.operator.key(), Errs::Op);

        if score > 50 {
            let mut i = 0;
            while i < score {
                if e.locked > 0 {
                    e.locked = e.locked.saturating_sub(1);
                }
                i = i.saturating_add(10);
            }
            e.score = e.score.saturating_add(score);
        } else {
            let mut j = 0;
            while j < 3 {
                e.locked = e.locked.saturating_add(1);
                j = j.saturating_add(1);
            }
            if comment.len() > 0 {
                e.score = e.score.saturating_add(comment.len() as u32);
            }
        }

        let mut amt = e.min.saturating_add((e.score as u64) / 2);
        if amt > e.locked {
            amt = e.locked;
        }
        e.released = e.released.saturating_add(amt);
        if e.locked >= amt {
            e.locked = e.locked - amt;
        }

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.vault.key(),
            ctx.accounts.player_ata.key(),
            ctx.accounts.operator.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.player_ata.to_account_info(),
            ctx.accounts.operator.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Escrow {
    pub operator: Pubkey,
    pub min: u64,
    pub locked: u64,
    pub released: u64,
    pub score: u32,
}

#[derive(Accounts)]
pub struct Make<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 4)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PassAndRelease<'info> {
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    pub operator: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub player_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}

#[error_code]
pub enum Errs {
    #[msg("operator mismatch")]
    Op,
}
