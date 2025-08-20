use anchor_lang::prelude::*;

declare_id!("MixChk1111111111111111111111111111111111");

#[program]
pub mod mixed_check1 {
    pub fn increment(ctx: Context<Inc>) -> Result<()> {
        // data.owner と signer.user の検証あり
        require_keys_eq!(ctx.accounts.data.owner, ctx.accounts.user.key(), CustomError::Unauthorized);
        // bounce_acc の所有者チェックが抜けている
        let data = &mut ctx.accounts.data;
        data.count = data.count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Inc<'info> {
    #[account(mut, has_one = owner)]
    pub data: Account<'info, CounterData>,
    pub owner: Signer<'info>,
    /// CHECK: 検証がないバウンス先
    #[account(mut)]
    pub bounce_acc: AccountInfo<'info>,
}

#[account]
pub struct CounterData {
    pub owner: Pubkey,
    pub count: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized")]
    Unauthorized,
}
