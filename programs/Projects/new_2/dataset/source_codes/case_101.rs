use anchor_lang::prelude::*;

declare_id!("VulnLam2222222222222222222222222222222222");

#[program]
pub mod vuln_lamport {
    pub fn sweep(ctx: Context<Sweep>) -> Result<()> {
        // attacker が任意のアカウントを指定できる
        let src = ctx.accounts.source.to_account_info();
        let dst = ctx.accounts.destination.to_account_info();
        **dst.lamports.borrow_mut() += **src.lamports.borrow();
        **src.lamports.borrow_mut() = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Sweep<'info> {
    /// CHECK: 所有者チェックがまったくない
    #[account(mut)]
    pub source: AccountInfo<'info>,
    #[account(mut)]
    pub destination: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
