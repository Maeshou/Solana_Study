use anchor_lang::prelude::*;

declare_id!("OwnerA11111111111111111111111111111111111");

#[program]
pub mod owner_delegator {
    use super::*;
    pub fn delegate_ownership(ctx: Context<DelegateOwnership>, new_owner_program_id: Pubkey) -> Result<()> {
        let account_to_delegate = &ctx.accounts.migratable_data.to_account_info();
        
        // アカウントの所有者を新しいプログラムIDに変更
        account_to_delegate.assign(&new_owner_program_id);
        
        // データの再割り当て（必要に応じて）
        // この操作はデータが消えるため注意が必要
        account_to_delegate.realloc(0, false)?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DelegateOwnership<'info> {
    // has_one制約で、この操作を行える権限者を検証
    #[account(
        mut,
        has_one = authority
    )]
    pub migratable_data: Account<'info, MigratableData>,
    pub authority: Signer<'info>,
}

#[account]
pub struct MigratableData {
    pub authority: Pubkey,
    pub data: u64,
}