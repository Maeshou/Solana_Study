use anchor_lang::prelude::*;

declare_id!("MixChkB011111111111111111111111111111111");

#[program]
pub mod mixed_check11 {
    pub fn distribute(ctx: Context<DistReward>) -> Result<()> {
        // reward.owner と署名者チェックあり
        require_keys_eq!(
            ctx.accounts.reward.owner,
            ctx.accounts.admin.key(),
            CustomError::Unauthorized
        );
        // vault_acc は所有者チェックなし
        let _ = ctx.accounts.vault_acc.data.borrow();
        // 本来ここで vault_acc から送金処理などを行う
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DistReward<'info> {
    #[account(mut, has_one = owner)]
    pub reward: Account<'info, RewardAccount>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub vault_acc: AccountInfo<'info>,
}

#[account]
pub struct RewardAccount {
    pub owner: Pubkey,
    pub total: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized")]
    Unauthorized,
}
