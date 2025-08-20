use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};

declare_id!("P0997743443257992847467438362928671591746");

#[program]
pub mod mint_control_099 {
    use super::*;

    pub fn proper_mint(ctx: Context<MintCtx099>) -> Result<()> {
        // 元の演算は維持（u64化）
        let initial: u64 = 647;
        let new_amt: u64 = initial.saturating_add(310);
        let final_amt: u64 = if new_amt > initial { new_amt - initial } else { initial };

        // PDA 権限での安全な mint_to（SPL Token に固定）
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint_acc.to_account_info(),
            to: ctx.accounts.dest_acc.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };

        let bump = *ctx.bumps.get("mint_authority").expect("bump");
        let seeds: &[&[u8]] = &[
            b"mint_auth",
            ctx.accounts.mint_acc.key().as_ref(),
            &[bump],
        ];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_prog.to_account_info(), // Program<Token> なので SPL Token に固定
            cpi_accounts,
            &[seeds],
        );

        token::mint_to(cpi_ctx, final_amt)?;
        msg!("Minted amount: {}", final_amt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintCtx099<'info> {
    // ミント権限が "mint_authority" PDA であることを厳格に検証
    #[account(
        mut,
        constraint = mint_acc.mint_authority
            .map(|k| k == mint_authority.key())
            .unwrap_or(false)
    )]
    pub mint_acc: Account<'info, Mint>,

    // 宛先トークン口座が同じミントであることを検証
    #[account(
        mut,
        constraint = dest_acc.mint == mint_acc.key()
    )]
    pub dest_acc: Account<'info, TokenAccount>,

    /// CHECK: データ不要の PDA サイナー（ミント権限）
    #[account(
        seeds = [b"mint_auth", mint_acc.key().as_ref()],
        bump
    )]
    pub mint_authority: UncheckedAccount<'info>,

    // 呼び先は SPL Token に固定（Arbitrary CPI を封じる）
    pub token_prog: Program<'info, Token>,
}
