use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("Que5tBoArD4444444444444444444444444444444");

#[program]
pub mod quest_board {
    use super::*;

    pub fn init_board(ctx: Context<InitBoard>) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.owner = ctx.accounts.owner.key();
        b.seed = 0xABCD;
        Ok(())
    }

    pub fn init_ticket(ctx: Context<InitTicket>, lane: u8) -> Result<()> {
        let t = &mut ctx.accounts.ticket;
        t.parent = ctx.accounts.board.key();
        t.lane = lane;
        t.used = false;
        Ok(())
    }

    pub fn run(ctx: Context<Run>, bonus: u16) -> Result<()> {
        require!(
            ctx.accounts.reward_ta.mint == ctx.accounts.reward_mint.key(),
            QuestErr::MintMismatch
        );
        require!(
            ctx.accounts.reward_ta.owner == ctx.accounts.owner.key(),
            QuestErr::OwnerMismatch
        );

        let b = &mut ctx.accounts.board;
        let a = &mut ctx.accounts.ticket_a;
        let c = &mut ctx.accounts.ticket_c;
        let r = &mut ctx.accounts.record;

        let mut sum = 0u32;
        for i in 0..6 {
            let v = ((bonus as u32) ^ (i as u32 * 17)) & 0x7FF;
            r.steps[i] = r.steps[i].saturating_add(v);
            sum = sum.saturating_add(v);
        }

        if (a.lane ^ c.lane) & 1 == 0 {
            a.used = true;
            r.ok = r.ok.saturating_add(sum / 8);
            b.seed = b.seed.rotate_left((sum % 16) as u32);
        } else {
            c.used = true;
            r.ng = r.ng.saturating_add(sum / 9);
            b.seed = b.seed.rotate_right((sum % 13) as u32);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4)]
    pub board: Account<'info, Board>,
    #[account(init, payer = owner, space = 8 + 4*6 + 8 + 8)]
    pub record: Account<'info, QRecord>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitTicket<'info> {
    #[account(mut)]
    pub board: Account<'info, Board>,
    #[account(init, payer = owner, space = 8 + 32 + 1 + 1)]
    pub ticket: Account<'info, TicketQ>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)]
    pub board: Account<'info, Board>,
    #[account(
        mut,
        has_one = parent,
        constraint = ticket_a.lane != ticket_c.lane @ QuestErr::CosplayBlocked
    )]
    pub ticket_a: Account<'info, TicketQ>,
    #[account(mut, has_one = parent)]
    pub ticket_c: Account<'info, TicketQ>,
    #[account(mut)]
    pub record: Account<'info, QRecord>,

    pub reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub reward_ta: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Board {
    pub owner: Pubkey,
    pub seed: u32,
}

#[account]
pub struct TicketQ {
    pub parent: Pubkey,
    pub lane: u8,
    pub used: bool,
}

#[account]
pub struct QRecord {
    pub steps: [u32; 6],
    pub ok: u64,
    pub ng: u64,
}

#[error_code]
pub enum QuestErr {
    #[msg("cosplay blocked")] CosplayBlocked,
    #[msg("mint mismatch")] MintMismatch,
    #[msg("owner mismatch")] OwnerMismatch,
}
