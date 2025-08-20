use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVM");

#[program]
pub mod token_vault {
    use super::*;

    /// Vault を初期化：指定ミントとオーナーを設定
    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner       = ctx.accounts.owner.key();
        vault.mint        = ctx.accounts.mint.key();
        vault.total_deposits = 0;
        Ok(())
    }

    /// トークンをデポジット：  
    /// - `user_ata` は `associated_token::…` 属性でミントとオーナーを検証  
    /// - Transfer CPI で入金  
    /// - 累計を更新  
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
    ) -> Result<()> {
        // SPL Token の Transfer CPI
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_ata.to_account_info(),
            to:   ctx.accounts.vault_ata.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        anchor_spl::token::transfer(cpi_ctx, amount)?;

        // 累計更新
        let vault = &mut ctx.accounts.vault;
        vault.total_deposits = vault.total_deposits.wrapping_add(amount);
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub owner:           Pubkey, // Vault オーナー
    pub mint:            Pubkey, // 扱うトークンの Mint
    pub total_deposits:  u64,    // 累計入金額
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    /// Vault データ
    #[account(
        init,
        payer = owner,
        space = 8   // discriminator
              +32  // owner
              +32  // mint
              +8   // total_deposits
    )]
    pub vault:       Account<'info, Vault>,

    /// 扱うトークンの Mint
    pub mint:        Account<'info, Mint>,

    /// Vault 用アソシエイテッドトークンアカウント（自動生成）
    #[account(
        init,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = vault,
    )]
    pub vault_ata:   Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner:       Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// 既存 Vault データ
    #[account(mut, has_one = owner, has_one = mint)]
    pub vault:       Account<'info, Vault>,

    /// 入金者の署名
    #[account(signer)]
    pub owner:       AccountInfo<'info>,

    /// 扱うトークンの Mint（一致チェック）
    pub mint:        Account<'info, Mint>,

    /// 入金者のアソシエイテッドトークンアカウント
    /// - mint が `vault.mint` と一致  
    /// - authority が `owner` と一致  
    #[account(
        mut,
        token::mint = vault.mint,
        token::authority = owner,
    )]
    pub user_ata:    Account<'info, TokenAccount>,

    /// Vault 用アソシエイテッドトークンアカウント
    #[account(mut)]
    pub vault_ata:   Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
