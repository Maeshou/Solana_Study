
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferNftCtxrajp<'info> {
    #[account(mut)] pub owner_record: Account<'info, DataAccount>,
    #[account(mut)] pub recipient: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_027 {
    use super::*;

    pub fn transfer_nft(ctx: Context<TransferNftCtxrajp>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.owner_record;
        // custom logic for transfer_nft
        assert!(ctx.accounts.owner_record.data > 0); acct.data -= amount;
        msg!("Executed transfer_nft logic");
        Ok(())
    }
}
