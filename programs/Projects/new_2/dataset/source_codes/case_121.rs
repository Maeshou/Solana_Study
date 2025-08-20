use anchor_lang::prelude::*;

declare_id!("MixChk2222222222222222222222222222222222");

#[program]
pub mod mixed_check2 {
    pub fn withdraw(ctx: Context<Withdraw>, amt: u64) -> Result<()> {
        // vault.owner と署名者チェックあり
        require_keys_eq!(ctx.accounts.vault.owner, ctx.accounts.manager.key(), CustomError::NotManager);
        // fee_acc は検証なし
        **ctx.accounts.recipient.lamports.borrow_mut() += amt;
        **ctx.accounts.vault_acc.lamports.borrow_mut() -= amt;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, VaultData>,
    pub owner: Signer<'info>,
    /// CHECK: fee_acc のオーナー未検証
    #[account(mut)]
    pub fee_acc: AccountInfo<'info>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
}

#[account]
pub struct VaultData {
    pub owner: Pubkey,
    pub balance: u64,
}
