use anchor_lang::prelude::*;

declare_id!("MarketRegistryAAAAABBBBBCCCCCDDDDDEEEEEFFFFF");

#[program]
pub mod market_registry {
    use super::*;

    pub fn register_seller(ctx: Context<Register>, flags: u8) -> Result<()> {
        let registry = &mut ctx.accounts.market_data;
        let account = &ctx.accounts.unknown;
        let note = &mut ctx.accounts.notes;

        note.data.borrow_mut()[0] = flags;
        registry.data.borrow_mut()[0] = account.key.as_ref()[0];

        if flags & 0x02 != 0 {
            registry.data.borrow_mut()[1] = 0xEE;
            note.data.borrow_mut()[1] = 0xEE;
        }

        if account.key.to_bytes()[0] % 2 == 0 {
            note.data.borrow_mut()[2] = 0xAA;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)]
    pub market_data: AccountInfo<'info>,
    #[account(mut)]
    pub unknown: AccountInfo<'info>, // Could be user or reviewer
    #[account(mut)]
    pub notes: AccountInfo<'info>,
}
