use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgAirDropSv01");

#[program]
pub mod airdrop_service {
    use super::*;

    /// エアドロップを請求するが、
    /// claim_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn claim_airdrop(ctx: Context<ClaimAirdrop>) -> Result<()> {
        let claim = &mut ctx.accounts.claim_account;
        let amount = ctx.accounts.config.airdrop_amount;

        // 1. 請求フラグを設定（所有者チェックなし）
        claim.claimed = true;

        // 2. エアドロッププールからユーザーへLamportsを直接送金
        **ctx.accounts.airdrop_vault.to_account_info().lamports.borrow_mut() -= amount;
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimAirdrop<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付けて所有者一致を検証すべき
    pub claim_account: Account<'info, ClaimAccount>,

    /// エアドロップ用プールアカウント
    #[account(mut)]
    pub airdrop_vault: AccountInfo<'info>,

    /// エアドロップを受け取るユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// エアドロップ量設定を保持するアカウント
    pub config: Account<'info, AirdropConfig>,
}

#[account]
pub struct ClaimAccount {
    /// 本来この請求を行うユーザーの Pubkey
    pub owner: Pubkey,
    /// 請求済みフラグ
    pub claimed: bool,
}

#[account]
pub struct AirdropConfig {
    /// 1回の請求で付与するLamports量
    pub airdrop_amount: u64,
}
