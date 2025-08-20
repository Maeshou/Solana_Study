use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf993mvTWf");

#[program]
pub mod validate_and_record_003 {
    use super::*;

    pub fn submit_text(ctx: Context<Ctx003>, content: String) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);

        // バリデーション: 空文字列・禁止語「hack」
        require!(!content.trim().is_empty(), CustomError::EmptyContent);
        require!(!content.contains("hack"), CustomError::BannedWord);

        // 長さを data に記録（例: 30文字なら30）
        let len = content.chars().count() as u64;
        ctx.accounts.storage.data = len;

        msg!("Text accepted. Length = {}, Author = {}", len, ctx.accounts.authority.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub data: u64, // テキスト長
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Submitted content is empty")]
    EmptyContent,
    #[msg("Banned word detected")]
    BannedWord,
}
