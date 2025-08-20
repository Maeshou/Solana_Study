
use anchor_lang::prelude::*;

declare_id!("Product333333333333333333333333333333333333");

#[program]
pub mod case3 {
    use super::*;

    pub fn transfer_ownership(ctx: Context<TransferOwnership>, new_owner: Pubkey, new_serial: u64) -> Result<()> {
        let product = &mut ctx.accounts.product;
        msg!("Transferring product from {:?} to {:?}", product.owner, new_owner);
        product.owner = new_owner;
        product.serial = new_serial;
        product.logs.push("Ownership and serial updated".into());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(mut)]
    pub product: Account<'info, Product>,
    /// CHECK: no authority or account validation
    pub sender: UncheckedAccount<'info>,
}

#[account]
pub struct Product {
    pub owner: Pubkey,
    pub serial: u64,
    pub logs: Vec<String>,
}
