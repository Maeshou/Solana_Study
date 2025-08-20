use anchor_lang::prelude::*;
use mpl_token_metadata::instruction as metadata_instruction;
use solana_program::program::invoke;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_050 {
    use super::*;
    pub fn update_meta(ctx: Context<MetaVuln050>) -> ProgramResult {
        let ix = metadata_instruction::update_metadata_accounts(
            &ctx.accounts.meta_prog.key(),
            &ctx.accounts.metadata.key(),
            None,
            None,
            Some(ctx.accounts.new_update_authority.key()),
        );
        // Arbitrary CPI without verifying meta program id
        invoke(&ix, &[
            ctx.accounts.meta_prog.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.new_update_authority.to_account_info(),
        ])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MetaVuln050<'info> {
    #[account(mut)] pub metadata: AccountInfo<'info>,
    pub new_update_authority: Signer<'info>,
    /// CHECK: metadata program unchecked
    pub meta_prog: UncheckedAccount<'info>,
}