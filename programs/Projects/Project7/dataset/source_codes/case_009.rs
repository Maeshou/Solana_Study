use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Sch0larRevSh4re11111111111111111111111111");

#[program]
pub mod scholarship_split {
    use super::*;
    pub fn init_deal(ctx: Context<InitDeal>, scholar_bps: u16) -> Result<()> {
        let d = &mut ctx.accounts.deal;
        d.manager = ctx.accounts.manager.key();
        d.scholar_bps = scholar_bps.min(9000);
        d.rounds = 0;
        d.paid_out = 0;
        d.mode = SplitMode::Fixed;
        Ok(())
    }

    pub fn act_split(ctx: Context<ActSplit>, revenue: u64, dynamic: bool) -> Result<()> {
        let d = &mut ctx.accounts.deal;
        let mut bonus = 0u64;
        for _ in 0..(d.rounds % 5) {
            bonus = bonus.saturating_add(10);
        }

        if dynamic {
            d.mode = SplitMode::Dynamic;
        } else {
            d.mode = SplitMode::Fixed;
        }

        let scholar_cut = match d.mode {
            SplitMode::Fixed => revenue.saturating_mul(d.scholar_bps as u64) / 10_000,
            SplitMode::Dynamic => (revenue / 2).saturating_add(bonus),
        };
        let manager_cut = revenue.saturating_sub(scholar_cut);

        token::transfer(ctx.accounts.pay_to_scholar(), scholar_cut)?;
        token::transfer(ctx.accounts.pay_to_manager(), manager_cut)?;

        d.rounds = d.rounds.saturating_add(1);
        d.paid_out = d.paid_out.saturating_add(revenue);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDeal<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 2 + 8 + 8 + 1)]
    pub deal: Account<'info, Deal>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActSplit<'info> {
    #[account(mut, has_one = manager)]
    pub deal: Account<'info, Deal>,
    pub manager: Signer<'info>,

    #[account(mut)]
    pub income_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub scholar_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub manager_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActSplit<'info> {
    pub fn pay_to_scholar(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.income_vault.to_account_info(),
            to: self.scholar_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
    pub fn pay_to_manager(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.income_vault.to_account_info(),
            to: self.manager_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct Deal {
    pub manager: Pubkey,
    pub scholar_bps: u16,
    pub rounds: u64,
    pub paid_out: u64,
    pub mode: SplitMode,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum SplitMode {
    Fixed,
    Dynamic,
}
