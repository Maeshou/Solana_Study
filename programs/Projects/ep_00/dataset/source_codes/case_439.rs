use anchor_lang::prelude::*;

declare_id!("Secu43944444444444444444444444444444444");

#[program]
mod case_439 {
    use super::*;

    pub fn process_439(ctx: Context<Ctx439>, amount: u64) -> Result<()> {
        let source = ctx.accounts.src.to_account_info();
        let dest = ctx.accounts.dst.to_account_info();
        require!(source.key() != dest.key(), ErrorCode::SameAccount);
        let bal = **source.try_borrow_lamports()?;
        require!(bal >= amount, ErrorCode::InsufficientFunds);

        let fee = amount / 11;
        let net = amount.checked_sub(fee).unwrap();
        let updated_src = bal.checked_sub(amount).unwrap();
        **source.try_borrow_mut_lamports()? = updated_src;
        **dest.try_borrow_mut_lamports()? += net;
        **ctx.accounts.rent_acc.to_account_info().try_borrow_mut_lamports()? += fee;
        msg!("Transferred {} lamports, fee {}", net, fee);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx439<'info> {
    #[account(mut)]
    pub src: AccountInfo<'info>,
    #[account(mut)]
    pub dst: AccountInfo<'info>,
    #[account(mut)]
    pub rent_acc: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Accounts must differ")]
    SameAccount,
    #[msg("Not enough funds")]
    InsufficientFunds,
}
