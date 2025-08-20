// 2) token_mint_to: 任意 program_id へ MintTo CPI
use anchor_lang::prelude::*;
use anchor_spl::token::spl_token;
use solana_program::program::invoke;

declare_id!("ArbCpiB222222222222222222222222222222222");

#[program]
pub mod token_mint_to {
    use super::*;

    pub fn reward(ctx: Context<Reward>, amount: u64) -> Result<()> {
        // リワード回数に応じた簡単な累積
        let s = &mut ctx.accounts.stats;
        if amount > 0 {
            s.count += 1;
        }

        // program_id 未固定の MintTo
        let ix = spl_token::instruction::mint_to(
            ctx.accounts.token_program.key(),              // ← 検証なし
            ctx.accounts.mint.key(),
            ctx.accounts.receiver_ata.key(),
            ctx.accounts.mint_authority.key(),
            &[],
            amount,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.receiver_ata.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct Stats {
    pub count: u64,
}

#[derive(Accounts)]
pub struct Reward<'info> {
    /// CHECK: 検証なし
    pub mint: UncheckedAccount<'info>,
    /// CHECK: 検証なし
    #[account(mut)]
    pub receiver_ata: UncheckedAccount<'info>,
    /// CHECK: 検証なし
    pub mint_authority: UncheckedAccount<'info>,
    /// CHECK: 検証なし（固定していない）
    pub token_program: UncheckedAccount<'info>,
    #[account(init_if_needed, payer = payer, space = 8 + 8)]
    pub stats: Account<'info, Stats>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
