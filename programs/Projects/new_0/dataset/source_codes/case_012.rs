use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf994mvTWf");

#[program]
pub mod whitelist_control_003 {
    use super::*;

    // 管理者が指定アドレスをホワイトリストとして登録
    pub fn register_whitelist(ctx: Context<Ctx003>, whitelist_key: Pubkey) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);
        ctx.accounts.storage.whitelisted = whitelist_key;
        msg!("Whitelisted address set to {}", whitelist_key);
        Ok(())
    }

    // 呼び出し者がホワイトリスト登録されたアドレスと一致するか確認
    pub fn verify_whitelist(ctx: Context<Ctx003>) -> Result<()> {
        let signer = ctx.accounts.authority.key();
        let expected = ctx.accounts.storage.whitelisted;
        require!(signer == expected, CustomError::NotWhitelisted);

        msg!("✅ Verified! {} is whitelisted.", signer);
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
    pub whitelisted: Pubkey,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Caller is not whitelisted")]
    NotWhitelisted,
}
