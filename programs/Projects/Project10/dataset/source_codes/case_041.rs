use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0410F88ID");

#[program]
pub mod commission_case_041 {
    use super::*;

    pub fn commission_action_041(ctx: Context<CommissionCtx041>, amount: u64) -> Result<()> {

        // Call external bridge
        let cpi_ctx = CpiContext::new(
            ctx.accounts.bridge_program.to_account_info(),
            ExternalBridge { from: ctx.accounts.src.to_account_info(), to: ctx.accounts.dst.to_account_info() });
        external_bridge(cpi_ctx)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CommissionCtx041<'info> {

    #[account(mut)]
    pub src: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dst: Account<'info, TokenAccount>,
    pub bridge_program: Program<'info, BridgeProgram>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"commission"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}


#[derive(Accounts)]
pub struct ExternalBridge<'info> {
    pub from: AccountInfo<'info>,
    pub to: AccountInfo<'info>,
}

extern "C" {
    fn external_bridge(ctx: CpiContext<ExternalBridge<'_>>) -> ProgramResult;
}


