// Program 10: rental_escrow （レンタル用エスクロー）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("RentalEscrowAAAAABBBBBCCCCCDDDDDEEEEEFFFF");

#[program]
pub mod rental_escrow {
    use super::*;

    pub fn init_escrow(ctx: Context<InitEscrow>, lease_id: u64) -> Result<()> {
        let e = &mut ctx.accounts.escrow;
        e.manager = ctx.accounts.manager.key();
        e.lease_id = lease_id.rotate_left(2).wrapping_add(33);
        e.state = 2;
        let mut i = 0u8;
        let mut s = e.lease_id.rotate_right(1).wrapping_add(5);
        while i < 4 {
            s = s.rotate_left(1).wrapping_mul(2).wrapping_add(7);
            if s % 3 > 0 { e.state = e.state.saturating_add(((s % 23) as u32) + 1); }
            i = i.saturating_add(1);
        }
        Ok(())
    }

    // 前金は System、保証は SPL で返却のような複合処理
    pub fn settle(ctx: Context<Settle>, rent: u64, token_refund: u64) -> Result<()> {
        let bump = *ctx.bumps.get("escrow").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"escrow", ctx.accounts.manager.key.as_ref(), &ctx.accounts.escrow.lease_id.to_le_bytes(), &[bump]];

        // 1) System: 家主へ家賃支払い
        let ix = system_instruction::transfer(&ctx.accounts.escrow.key(), &ctx.accounts.landlord.key(), rent);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.escrow.to_account_info(),
                ctx.accounts.landlord.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        // 2) SPL: 借主に保証トークン返却
        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_token.to_account_info(),
            to: ctx.accounts.tenant_token.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, &[seeds]);

        // 分割返却ロジック
        let mut left = token_refund;
        let mut step = (token_refund / 3).max(1);
        let mut rounds = 3u8;
        while left > 0 && rounds > 0 {
            let give = left.min(step);
            token::transfer(cpi_ctx.clone(), give)?;
            left = left.saturating_sub(give);
            step = step.rotate_right(1).wrapping_add(2);
            if step > left && left > 4 { step = left - 2; }
            rounds = rounds.saturating_sub(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(
        init,
        payer = manager,
        space = 8 + 32 + 8 + 4,
        seeds=[b"escrow", manager.key().as_ref(), lease_id.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub lease_id: u64,
}

#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(
        mut,
        seeds=[b"escrow", manager.key().as_ref(), escrow.lease_id.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub landlord: SystemAccount<'info>,
    #[account(mut)]
    pub escrow_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub tenant_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Escrow {
    pub manager: Pubkey,
    pub lease_id: u64,
    pub state: u32,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
