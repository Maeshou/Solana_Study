use anchor_lang::prelude::*;
use solana_program::program::invoke_signed;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_015 {
    use super::*;
    pub fn mint_tokens(ctx: Context<MintVuln015>, amount: u64) -> ProgramResult {
        // Build mint_to instruction
        let instructions = spl_token::instruction::mint_to(
            &ctx.accounts.mint_prog.key(),
            &ctx.accounts.mint_acc.key(),
            &ctx.accounts.dest_acc.key(),
            &ctx.accounts.mint_authority.key(),
            &[],
            amount,
        )?;
        // Using invoke_signed but not verifying the program ID
        invoke_signed(
            &instructions,
            &[
                ctx.accounts.mint_prog.to_account_info(),
                ctx.accounts.mint_acc.to_account_info(),
                ctx.accounts.dest_acc.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
            ],
            &[],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintVuln015<'info> {
    #[account(mut)] pub mint_acc: AccountInfo<'info>,
    #[account(mut)] pub dest_acc: AccountInfo<'info>,
    pub mint_authority: Signer<'info>,
    /// CHECK: unchecked mint program
    pub mint_prog: UncheckedAccount<'info>,
}