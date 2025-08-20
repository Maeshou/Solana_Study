use anchor_lang::prelude::*;

declare_id!("00000000000000000000000000000000");

#[program]
pub mod init_data_account {
    use super::*;

    pub fn init_data(
        ctx: Context<InitDataAccount>,
        data: Vec<u8>,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.data_account;
        acct.data = data;
        acct.processed = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDataAccount<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: Vec<u8>,
    pub processed: bool,
}
