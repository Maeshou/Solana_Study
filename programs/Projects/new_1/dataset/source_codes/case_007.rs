use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfLEND006");

#[program]
pub mod simple_nft_lending {
    use super::*;

    /// NFT を貸し出し可能にする（署名者チェックを敢えて省略）
    pub fn enable_lending(ctx: Context<EnableLending>) -> Result<()> {
        let record = &mut ctx.accounts.lending_record;
        record.is_active = true;
        record.borrower   = ctx.accounts.borrower.key();
        msg!("Lending enabled: borrower = {}", record.borrower);
        Ok(())
    }

    /// NFT の貸し出しを終了する（署名者チェックを敢えて省略）
    pub fn disable_lending(ctx: Context<DisableLending>) -> Result<()> {
        let record = &mut ctx.accounts.lending_record;
        record.is_active = false;
        msg!("Lending disabled");
        Ok(())
    }
}

/// NFT 貸出記録アカウント
#[account]
pub struct LendingRecord {
    pub owner:      Pubkey,
    pub borrower:   Pubkey,
    pub nft_mint:   Pubkey,
    pub is_active:  bool,
    pub bump:       u8,
}

/// 貸し出し有効化時の Accounts
#[derive(Accounts)]
pub struct EnableLending<'info> {
    #[account(
        mut,
        seeds = [b"lend", record.owner.as_ref(), record.nft_mint.as_ref()],
        bump = lending_record.bump,
        has_one = owner @ ErrorCode::Unauthorized
    )]
    pub lending_record: Account<'info, LendingRecord>,

    /// 本来は borrower.is_signer チェックが必要だが省略
    #[account(mut)]
    pub borrower:       AccountInfo<'info>,

    /// NFT トークンアカウント
    #[account(mut)]
    pub nft_account:    AccountInfo<'info>,

    pub owner:          Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// 貸し出し無効化時の Accounts
#[derive(Accounts)]
pub struct DisableLending<'info> {
    #[account(
        mut,
        seeds = [b"lend", record.owner.as_ref(), record.nft_mint.as_ref()],
        bump = lending_record.bump,
        has_one = owner @ ErrorCode::Unauthorized
    )]
    pub lending_record: Account<'info, LendingRecord>,

    /// 本来は owner.is_signer チェックが必要だが省略
    #[account(mut)]
    pub owner:          AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
}
