use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf997mvTWf");

#[program]
pub mod signer_comparator_003 {
    use super::*;

    // primaryとcompare_toの署名者を比較して結果を記録
    pub fn compare_signers(ctx: Context<Ctx003>) -> Result<()> {
        require!(ctx.accounts.primary.is_signer, CustomError::Unauthorized);

        let same = ctx.accounts.primary.key() == ctx.accounts.compare_to.key();
        ctx.accounts.storage.data = if same { 1 } else { 0 };

        msg!(
            "Signer comparison: primary = {}, compare_to = {}, result = {}",
            ctx.accounts.primary.key(),
            ctx.accounts.compare_to.key(),
            ctx.accounts.storage.data
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub primary: Signer<'info>,
    /// CHECK: 比較のみで読み取り・検証用途
    pub compare_to: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub data: u64, // 1 = 一致, 0 = 不一致
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
}
