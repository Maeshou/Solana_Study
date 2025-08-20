use anchor_lang::prelude::*;
use solana_program::program::invoke_signed;

declare_id!("P0854264486479948721927888443726317399418");

#[program]
pub mod insecure_mint_085 {
    use super::*;

    pub fn improper_mint(ctx: Context<MintCtx085>) -> ProgramResult {
        // Split with fee distribution
        let total_amount = 830;
        let fee = total_amount * 15 / 100;
        let net = total_amount - fee;
        let part1 = net / 3;
        let part2 = net - part1;
        // Mint fee to treasury
        let ix_fee = spl_token::instruction::mint_to(
            &ctx.accounts.token_prog.key(),
            &ctx.accounts.mint_acc.key(),
            &ctx.accounts.treasury_acc.key(),
            &ctx.accounts.authority.key(),
            &[],
            fee,
        )?;
        invoke_signed(&ix_fee, &[
            ctx.accounts.token_prog.to_account_info(),
            ctx.accounts.mint_acc.to_account_info(),
            ctx.accounts.treasury_acc.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        ], &[])?;
        // Mint part1 to recipient1
        let ix1 = spl_token::instruction::mint_to(
            &ctx.accounts.token_prog.key(),
            &ctx.accounts.mint_acc.key(),
            &ctx.accounts.recipient1.key(),
            &ctx.accounts.authority.key(),
            &[],
            part1,
        )?;
        invoke_signed(&ix1, &[
            ctx.accounts.token_prog.to_account_info(),
            ctx.accounts.mint_acc.to_account_info(),
            ctx.accounts.recipient1.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        ], &[])?;
        // Mint part2 to recipient2
        let ix2 = spl_token::instruction::mint_to(
            &ctx.accounts.token_prog.key(),
            &ctx.accounts.mint_acc.key(),
            &ctx.accounts.recipient2.key(),
            &ctx.accounts.authority.key(),
            &[],
            part2,
        )?;
        invoke_signed(&ix2, &[
            ctx.accounts.token_prog.to_account_info(),
            ctx.accounts.mint_acc.to_account_info(),
            ctx.accounts.recipient2.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        ], &[])?;
        msg!("Distributed with fee {} to treasury, remaining {} split", fee, net);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintCtx085<'info> {
    #[account(mut, has_one = auth)]
    pub mint_acc: Account<'info, Mint>,
    #[account(mut)]
    pub dest_acc: Account<'info, TokenAccount>,
    #[account(signer)]
    pub auth: AccountInfo<'info>,
    /// CHECK: token program unchecked
    pub token_prog: UncheckedAccount<'info>,
}
