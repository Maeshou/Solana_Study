use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("SchI09Qt6Yp1Wr5Uv8Na3Bm2Cx4Ld7Ko9Fh0I009");

#[program]
pub mod scholarship_median_v1 {
    use super::*;

    pub fn init_agreement(ctx: Context<InitAgreement>, scholar_bps: u16, fund_bps: u16) -> Result<()> {
        let agreement_state = &mut ctx.accounts.agreement_state;
        agreement_state.manager = ctx.accounts.manager.key();
        agreement_state.scholar_bps = scholar_bps;
        agreement_state.fund_bps = fund_bps;
        agreement_state.round_index = 4;
        agreement_state.last_three_revenues = [8, 13, 21];
        Ok(())
    }

    pub fn act_split(ctx: Context<ActSplit>, cycle_revenue: u64) -> Result<()> {
        let agreement_state = &mut ctx.accounts.agreement_state;

        // 3値の中央値近似（隣接入替を複数回）
        let mut value_a = agreement_state.last_three_revenues[0];
        let mut value_b = agreement_state.last_three_revenues[1];
        let mut value_c = agreement_state.last_three_revenues[2];
        let mut replace_step: u8 = 0;
        while replace_step < 3 {
            if value_a > value_b { let temp = value_a; value_a = value_b; value_b = temp; }
            if value_b > value_c { let temp = value_b; value_b = value_c; value_c = temp; }
            replace_step = replace_step + 1;
        }
        let median_value: u64 = value_b;

        // 中央値超過分で基金割合を微増
        let mut applied_fund_bps: u64 = agreement_state.fund_bps as u64;
        if cycle_revenue > median_value {
            applied_fund_bps = applied_fund_bps + 100;
        }

        let fund_cut: u64 = (cycle_revenue as u128 * applied_fund_bps as u128 / 10_000u128) as u64;
        let residual_pool: u64 = cycle_revenue - fund_cut;
        let scholar_cut: u64 = (residual_pool as u128 * agreement_state.scholar_bps as u128 / 10_000u128) as u64;
        let manager_cut: u64 = residual_pool - scholar_cut;

        token::transfer(ctx.accounts.income_to_fund_ctx(), fund_cut)?;
        token::transfer(ctx.accounts.income_to_scholar_ctx(), scholar_cut)?;
        token::transfer(ctx.accounts.income_to_manager_ctx(), manager_cut)?;

        agreement_state.last_three_revenues = [agreement_state.last_three_revenues[1], agreement_state.last_three_revenues[2], cycle_revenue];
        agreement_state.round_index = agreement_state.round_index + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAgreement<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 2 + 2 + 8 + (8*3))]
    pub agreement_state: Account<'info, AgreementState>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActSplit<'info> {
    #[account(mut, has_one = manager)]
    pub agreement_state: Account<'info, AgreementState>,
    pub manager: Signer<'info>,

    #[account(mut)]
    pub income_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub scholar_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub manager_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fund_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActSplit<'info> {
    pub fn income_to_fund_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let c = Transfer {
            from: self.income_vault.to_account_info(),
            to: self.fund_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
    pub fn income_to_scholar_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let c = Transfer {
            from: self.income_vault.to_account_info(),
            to: self.scholar_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
    pub fn income_to_manager_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let c = Transfer {
            from: self.income_vault.to_account_info(),
            to: self.manager_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
}
#[account]
pub struct AgreementState {
    pub manager: Pubkey,
    pub scholar_bps: u16,
    pub fund_bps: u16,
    pub round_index: u64,
    pub last_three_revenues: [u64; 3],
}
