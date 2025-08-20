// Secure Case 290: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID290XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_290 {
    pub fn perform_trade(ctx: Context<Accounts290>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error290::InvalidOwner);
        
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
pub struct Accounts290<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct290>,
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
pub struct DataStruct290 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error290 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
