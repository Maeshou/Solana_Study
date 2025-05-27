use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000010");

#[program]
pub mod case016_ticket_sale {
    use super::*;

    pub fn initialize_ticket_sale(ctx: Context<Initialize016>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = TicketSaleData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize016<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TicketSaleData {
    pub value: u64,
}
