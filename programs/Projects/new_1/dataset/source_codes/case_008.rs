use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfVEST007");

#[program]
pub mod token_vesting {
    use super::*;

    /// トークンをロックしてベストアカウントを初期化します（署名者チェックを敢えて省略）
    pub fn lock_tokens(ctx: Context<LockTokens>, amount: u64) -> Result<()> {
        let vest = &mut ctx.accounts.vest_account;
        vest.owner        = ctx.accounts.token_owner.key();
        vest.beneficiary  = ctx.accounts.beneficiary.key();
        vest.total_amount = amount;
        vest.claimed      = 0;
        msg!("Locked {} tokens for {}", amount, vest.beneficiary);
        Ok(())
    }

    /// ロックされたトークンを請求します（署名者チェックを敢えて省略）
    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
        let vest = &mut ctx.accounts.vest_account;
        let to_claim = vest.total_amount
            .checked_sub(vest.claimed)
            .unwrap();
        vest.claimed = vest.total_amount;
        // 本来は beneficiary.is_signer チェックが必要
        msg!("Claimed {} tokens for {}", to_claim, ctx.accounts.beneficiary.key());
        Ok(())
    }
}

/// ベスティング記録アカウント
#[account]
pub struct VestAccount {
    pub owner:        Pubkey,
    pub beneficiary:  Pubkey,
    pub total_amount: u64,
    pub claimed:      u64,
    pub bump:         u8,
}

/// トークンロック時の Accounts
#[derive(Accounts)]
pub struct LockTokens<'info> {
    #[account(
        init,
        payer = token_owner,
        space = 8 + 32 + 32 + 8 + 8 + 1,
        seeds = [b"vest", beneficiary.key().as_ref()],
        bump
    )]
    pub vest_account:  Account<'info, VestAccount>,

    /// トークン所有者（本来は署名必須）
    #[account(mut)]
    pub token_owner:   AccountInfo<'info>,

    /// ベネフィシャリー（本来は署名必須）
    #[account(mut)]
    pub beneficiary:   AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

/// トークン請求時の Accounts
#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(
        mut,
        seeds = [b"vest", beneficiary.key().as_ref()],
        bump = vest_account.bump,
        has_one = owner @ ErrorCode::Unauthorized
    )]
    pub vest_account:  Account<'info, VestAccount>,

    /// ベネフィシャリー（本来は署名必須）
    #[account(mut)]
    pub beneficiary:   AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
}
