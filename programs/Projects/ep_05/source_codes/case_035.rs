use anchor_lang::prelude::*;

declare_id!("Secu03555555555555555555555555555555555");

#[program]
pub mod case035 {
    use super::*;

    pub fn process_035(ctx: Context<Ctx035>) -> Result<()> {
        let a = ctx.accounts.acc_a.to_account_info();
let b = ctx.accounts.acc_b.to_account_info();
require!(a.key() != b.key(), ErrorCode::DuplicateAccount);
let bal = **a.try_borrow_lamports()?;
require!(bal >= 100, ErrorCode::InsufficientResources);
let fee = bal.checked_div(3).unwrap();
let net = bal.checked_sub(fee).unwrap();
**a.try_borrow_mut_lamports()? = net;
**b.try_borrow_mut_lamports()? += fee;
msg!("Transferred {} lamports with fee {}", net, fee);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx035<'info> {
    #[account(mut)]
    pub acc_a: AccountInfo<'info>,
    #[account(mut)]
    pub acc_b: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    
}

#[error_code]
pub enum ErrorCode {
    #[msg("Accounts must differ")]
    DuplicateAccount,
    #[msg("Insufficient resources")]
    InsufficientResources,
}
