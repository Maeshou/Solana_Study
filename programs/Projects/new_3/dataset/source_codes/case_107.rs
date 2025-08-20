use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf107mvTWf");

#[program]
pub mod validate_data_107 {
    use super::*;

    pub fn validate_data(ctx: Context<Ctx107>) -> Result<()> {
        let old_pub = ctx.accounts.rec.data_pub;
        let new_pub = ctx.accounts.user.key();
        ctx.accounts.rec.data_pub = new_pub;
        msg!("Case 107: data_pub changed from {} to {}", old_pub, new_pub);
        Ok(())
    }
}
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, MintTo, Burn, CpiContext, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgFractRematch01");

#[program]
pub mod fractionalization_service {
    use super::*;

    /// NFT をフラクショナル化して分割トークンをミントするが、
    /// has_one = fraction_mint のみ検証し、アカウントの所有者（owner）とは照合していない
    pub fn fractionalize(
        ctx: Context<Fractionalize>,
        fraction_amount: u64,
    ) -> Result<()> {
        let frac_acc = &mut ctx.accounts.fraction_account;

        // 1. 累計分割トークン数を更新
        frac_acc.total_fractions = frac_acc.total_fractions
            .checked_add(fraction_amount)
            .unwrap();

        // 2. 分割トークンをユーザーにミント
        let cpi_accounts = MintTo {
            mint: ctx.accounts.fraction_mint.to_account_info(),
            to: ctx.accounts.user_fraction_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, fraction_amount)?;

        Ok(())
    }

    /// 分割トークンをバーンして NFT を再構築するが、
    /// has_one = fraction_mint のみ検証し、アカウントの所有者（owner）とは照合していない
    pub fn defractionalize(
        ctx: Context<Defractionalize>,
        burn_amount: u64,
    ) -> Result<()> {
        let frac_acc = &mut ctx.accounts.fraction_account;

        // 1. 累計分割トークン数をデクリメント
        frac_acc.total_fractions = frac_acc.total_fractions
            .checked_sub(burn_amount)
            .unwrap();

        // 2. 分割トークンをバーン
        let cpi_accounts = Burn {
            mint: ctx.accounts.fraction_mint.to_account_info(),
            from: ctx.accounts.user_fraction_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::burn(cpi_ctx, burn_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Fractionalize<'info> {
    #[account(
        mut,
        has_one = fraction_mint, // ミントアドレスだけ検証
        // 本来は has_one = owner を追加して所有者照合すべき
    )]
    pub fraction_account: Account<'info, FractionAccount>,

    /// 分割トークンの Mint
    pub fraction_mint: Account<'info, Mint>,

    /// ユーザーの分割トークン受取 TokenAccount
    #[account(mut)]
    pub user_fraction_account: Account<'info, TokenAccount>,

    /// Mint 権限を持つアカウント
    pub mint_authority: Signer<'info>,

    /// SPL Token プログラム
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Defractionalize<'info> {
    #[account(
        mut,
        has_one = fraction_mint, // ミントアドレスだけ検証
        // 本来は has_one = owner を追加して所有者照合すべき
    )]
    pub fraction_account: Account<'info, FractionAccount>,

    /// 分割トークンの Mint
    pub fraction_mint: Account<'info, Mint>,

    /// ユーザーの分割トークン保有 TokenAccount
    #[account(mut)]
    pub user_fraction_account: Account<'info, TokenAccount>,

    /// SPL Token プログラム
    pub token_program: Program<'info, Token>,

    /// ユーザー（バーン権限）
    pub user: Signer<'info>,
}

#[account]
pub struct FractionAccount {
    /// 本来このフラクショナル化権を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 対応する分割トークンの Mint
    pub fraction_mint: Pubkey,
    /// これまでにミントされた分割トークンの総数
    pub total_fractions: u64,
}

#[derive(Accounts)]
pub struct Ctx107<'info> {
    #[account(mut, has_one = owner)]
    pub rec: Account<'info, Rec107>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Rec107 {
    pub owner: Pubkey,
    pub data_pub: Pubkey,
}
