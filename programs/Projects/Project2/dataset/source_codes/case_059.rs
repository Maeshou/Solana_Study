use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Lend111111111111111111111111111111111111");

#[program]
pub mod lending_protocol {
    use super::*;
    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_collateral.to_account_info(),
                    to: ctx.accounts.collateral_vault.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    // マーケットアカウント（自プログラム所有）
    #[account(seeds = [b"market"], bump)]
    pub market: Account<'info, Market>,

    // 担保を保管するVault（所有者はmarket PDA）
    #[account(mut, token::authority = market)]
    pub collateral_vault: Account<'info, TokenAccount>,
    
    // ユーザーの担保アカウント（所有者はToken Program）
    #[account(mut)]
    pub user_collateral: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Market {
    pub admin: Pubkey,
}