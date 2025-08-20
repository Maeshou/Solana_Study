use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzT4");

#[program]
pub mod secure_vault {
    use super::*;

    /// 初期化：vault を PDA で生成し、authority と bump を保存
    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        bump: u8,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.bump = bump;
        Ok(())
    }

    /// オーナー変更：現在の authority のサイン必須、has_one で一致保証
    pub fn update_authority(
        ctx: Context<UpdateAuthority>,
        new_authority: Pubkey,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        // has_one + signer で検証済み
        vault.authority = new_authority;
        Ok(())
    }

    /// 引き出し：残高チェック＆オーバーフロー防止
    pub fn withdraw(
        ctx: Context<WithdrawFunds>,
        amount: u64,
    ) -> Result<()> {
        let vault_info = ctx.accounts.vault.to_account_info();
        let recipient_info = ctx.accounts.recipient.to_account_info();

        // 現在の残高を取得
        let vault_balance = **vault_info.try_borrow_lamports()?;
        // 残高不足チェック
        let new_vault_balance = vault_balance
            .checked_sub(amount)
            .ok_or(ErrorCode::InsufficientFunds)?;
        // オーバーフロー防止
        let recipient_balance = **recipient_info.try_borrow_lamports()?;
        let new_recipient_balance = recipient_balance
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;

        // 実際の lamports 更新
        **vault_info.try_borrow_mut_lamports()? = new_vault_balance;
        **recipient_info.try_borrow_mut_lamports()? = new_recipient_balance;

        Ok(())
    }
}

/// 初期化時に使うアカウント定義
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1,  // discriminator + Pubkey + bump
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// オーナー変更：has_one + signer でチェック
#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,

    /// 現在のオーナー（署名あり）
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

/// 引き出し：has_one + signer でチェック
#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,

    /// 引き出し先（任意のウォレット）
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    /// 現在のオーナー（署名あり）
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

/// Vault のデータ構造：authority と bump を保持
#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub bump: u8,
}

/// カスタムエラー定義
#[error_code]
pub enum ErrorCode {
    #[msg("Authority signature is required")]
    MissingSigner,
    #[msg("Insufficient funds to withdraw")]
    InsufficientFunds,
    #[msg("Balance overflow occurred")]
    Overflow,
}
