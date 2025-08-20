use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0832361ID");

#[program]
pub mod escrow_deposit_case_083 {
    use super::*;

    pub fn escrow_deposit_action_083(ctx: Context<Escrow_depositCtx083>, amount: u64) -> Result<()> {

        // Call external bridge
        let cpi_ctx = CpiContext::new(
            ctx.accounts.bridge_program.to_account_info(),
            ExternalBridge { from: ctx.accounts.src.to_account_info(), to: ctx.accounts.dst.to_account_info() });
        external_bridge(cpi_ctx)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Escrow_depositCtx083<'info> {

    #[account(mut)]
    pub src: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dst: Account<'info, TokenAccount>,
    pub bridge_program: Program<'info, BridgeProgram>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"escrow_deposit"], bump)]
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


