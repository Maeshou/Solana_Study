use anchor_lang::prelude::*;

declare_id!("Var2Inv2222222222222222222222222222222222");

#[program]
pub mod varied_inventory {
    use super::*;

    pub fn init_stock(ctx: Context<InitStock>, initial: u32) -> Result<()> {
        let s = &mut ctx.accounts.stock;
        s.quantity = initial;
        Ok(())
    }

    pub fn consume(ctx: Context<Consume>, amount: u32) -> Result<()> {
        let mut left = ctx.accounts.stock.quantity;
        let mut to_take = amount;
        
        // 「&&」を避けて、消費ループを while と内部 if で分割
        while to_take > 0 {
            if left == 0 {
                break;
            }
            left -= 1;
            to_take -= 1;
        }
        
        let c = &mut ctx.accounts.consumption;
        c.taken = amount - to_take;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStock<'info> {
    #[account(init, payer = manager, space = 8 + 4)]
    pub stock: Account<'info, StockData>,
    #[account(mut)] pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Consume<'info> {
    pub stock: Account<'info, StockData>,
    #[account(mut, init, payer = manager, space = 8 + 4)]
    pub consumption: Account<'info, ConsumptionData>,
    #[account(mut)] pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StockData {
    pub quantity: u32,
}

#[account]
pub struct ConsumptionData {
    pub taken: u32,
}
