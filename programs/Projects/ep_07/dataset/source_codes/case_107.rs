use anchor_lang::prelude::*;
use solana_program::program::invoke_signed;

declare_id!("P0462714939112611395962786388289274698532");

#[program]
pub mod insecure_mint_046 {
    use super::*;

    pub fn improper_mint(ctx: Context<MintCtx046>) -> ProgramResult {
        // Saturating add/sub
        let initial = 709;
        let new_amt = initial.saturating_add(235);
        let final_amt = if new_amt > initial { new_amt - initial } else { initial };
        let ix = spl_token::instruction::mint_to(
            &ctx.accounts.token_prog.key(),
            &ctx.accounts.mint_acc.key(),
            &ctx.accounts.dest_acc.key(),
            &ctx.accounts.auth.key(),
            &[],
            final_amt,
        )?;
        invoke_signed(&ix, &[
            ctx.accounts.token_prog.to_account_info(),
            ctx.accounts.mint_acc.to_account_info(),
            ctx.accounts.dest_acc.to_account_info(),
            ctx.accounts.auth.to_account_info(),
        ], &[])?;
        msg!("Saturating minted amount: {}", final_amt);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintCtx046<'info> {
    #[account(mut, has_one = auth)]
    pub mint_acc: Account<'info, Mint>,
    #[account(mut)]
    pub dest_acc: Account<'info, TokenAccount>,
    #[account(signer)]
    pub auth: AccountInfo<'info>,
    /// CHECK: token program unchecked
    pub token_prog: UncheckedAccount<'info>,
}
