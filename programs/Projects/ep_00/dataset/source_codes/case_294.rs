// Secure Case 294: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID294XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_294 {
    pub fn run_serialization(ctx: Context<Accounts294>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error294::InvalidOwner);
        
        let src = ctx.accounts.src_acc.to_account_info();
        let dst = ctx.accounts.dst_acc.to_account_info();
        let src_bal = **src.lamports.borrow();
        let half = src_bal / 2;
        **src.try_borrow_mut_lamports()? = src_bal - half;
        **dst.try_borrow_mut_lamports()? += half;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts294<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct294>,
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
pub struct DataStruct294 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error294 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
