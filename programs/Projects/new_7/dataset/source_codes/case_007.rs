// 4) raid_bounty_board
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Ra1dB0untyB04rd000000000000000000000004");

#[program]
pub mod raid_bounty_board {
    use super::*;

    pub fn spawn(ctx: Context<Spawn>, base: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.master = ctx.accounts.master.key();
        b.base = base;
        b.entries = 0;
        b.denied = 0;
        b.total = 0;
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, weight: u8, proof: String) -> Result<()> {
        let b = &mut ctx.accounts.board;
        require!(b.master == ctx.accounts.master.key(), Errs::Master);

        // 証拠文字列に応じた調整（両分岐を長めに）
        if proof.len() > 8 {
            let mut sum = 0u64;
            let mut i = 0;
            while i < proof.as_bytes().len() {
                sum = sum.saturating_add(proof.as_bytes()[i] as u64);
                i += 1;
            }
            b.total = b.total.saturating_add(sum % 100);
            b.entries = b.entries.saturating_add(1);
        } else {
            let mut p = 0;
            while p < 5 {
                if b.denied < 100 {
                    b.denied = b.denied.saturating_add(1);
                }
                p = p.saturating_add(1);
            }
        }

        let mut amt = b.base.saturating_mul(weight as u64);
        if amt > 0 {
            let mut lift = 0;
            while lift < weight {
                amt = amt.saturating_add(1);
                lift = lift.saturating_add(1);
            }
        }

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.treasury.key(),
            ctx.accounts.hunter_ata.key(),
            ctx.accounts.master.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.treasury.to_account_info(),
            ctx.accounts.hunter_ata.to_account_info(),
            ctx.accounts.master.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Board {
    pub master: Pubkey,
    pub base: u64,
    pub entries: u32,
    pub denied: u32,
    pub total: u64,
}

#[derive(Accounts)]
pub struct Spawn<'info> {
    #[account(init, payer = master, space = 8 + 32 + 8 + 4 + 4 + 8)]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub board: Account<'info, Board>,
    pub master: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub hunter_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}

#[error_code]
pub enum Errs {
    #[msg("master mismatch")]
    Master,
}
