// Secure Case 228: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID228XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_228 {
    pub fn handle_claim(ctx: Context<Accounts228>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error228::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        **info.try_borrow_mut_lamports()? = amount;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts228<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct228>,
    #[account(mut)]
    pub account: AccountInfo<'info>,
    #[account(mut)]
    pub src_acc: AccountInfo<'info>,
    #[account(mut)]
    pub dst_acc: AccountInfo<'info>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct DataStruct228 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error228 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
