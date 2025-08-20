use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("StaMiNaP45522222222222222222222222222222");

#[program]
pub mod stamina_pass {
    use super::*;

    pub fn init_runner(ctx: Context<InitRunner>) -> Result<()> {
        let r = &mut ctx.accounts.runner;
        r.owner = ctx.accounts.owner.key();
        r.energy = 50;
        r.level = 1;
        Ok(())
    }

    pub fn issue_pass(ctx: Context<IssuePass>, slot: u8) -> Result<()> {
        let p = &mut ctx.accounts.pass;
        p.parent = ctx.accounts.runner.key();
        p.slot = slot;
        p.consumed = false;
        Ok(())
    }

    pub fn recharge(ctx: Context<Recharge>, add: u16) -> Result<()> {
        let r = &mut ctx.accounts.runner;
        let a = &mut ctx.accounts.pass_a;
        let b = &mut ctx.accounts.pass_b;
        let g = &mut ctx.accounts.gauge;

        require!(
            ctx.accounts.fuel_ta.mint == ctx.accounts.fuel_mint.key(),
            RunErr::MintMismatch
        );
        require!(
            ctx.accounts.fuel_ta.owner == ctx.accounts.owner.key(),
            RunErr::OwnerMismatch
        );

        let mut sum = 0u32;
        for i in 0..g.cells.len() {
            let inc = (add as u32).saturating_add((i as u32) * 5);
            g.cells[i] = g.cells[i].saturating_add(inc);
            sum = sum.saturating_add(g.cells[i]);
        }

        if sum & 1 == 1 {
            r.energy = r.energy.saturating_add((sum / 20) as u64);
            a.consumed = true;
            g.flag = true;
        } else {
            r.energy = r.energy.saturating_sub((sum / 25) as u64);
            b.consumed = true;
            g.flag = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRunner<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 2)]
    pub runner: Account<'info, Runner>,
    #[account(init, payer = owner, space = 8 + 1 + 4*4 + 1)]
    pub gauge: Account<'info, Gauge>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IssuePass<'info> {
    #[account(mut)]
    pub runner: Account<'info, Runner>,
    #[account(init, payer = owner, space = 8 + 32 + 1 + 1)]
    pub pass: Account<'info, Pass>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Recharge<'info> {
    #[account(mut)]
    pub runner: Account<'info, Runner>,
    #[account(
        mut,
        has_one = parent,
        constraint = pass_a.slot != pass_b.slot @ RunErr::CosplayBlocked
    )]
    pub pass_a: Account<'info, Pass>,
    #[account(mut, has_one = parent)]
    pub pass_b: Account<'info, Pass>,
    #[account(mut)]
    pub gauge: Account<'info, Gauge>,

    pub fuel_mint: Account<'info, Mint>,
    #[account(mut)]
    pub fuel_ta: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Runner {
    pub owner: Pubkey,
    pub energy: u64,
    pub level: u16,
}

#[account]
pub struct Pass {
    pub parent: Pubkey,
    pub slot: u8,
    pub consumed: bool,
}

#[account]
pub struct Gauge {
    pub flag: bool,
    pub cells: [u32; 4],
    pub reserved: u8,
}

#[error_code]
pub enum RunErr {
    #[msg("type cosplay blocked")] CosplayBlocked,
    #[msg("mint mismatch")] MintMismatch,
    #[msg("owner mismatch")] OwnerMismatch,
}
