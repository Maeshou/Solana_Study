use anchor_lang::prelude::*;

declare_id!("SafeEx07XXXXXXX7777777777777777777777777777");

#[program]
pub mod example7 {
    use super::*;

    pub fn init_event(
        ctx: Context<InitEvent>,
        capacity: u32,
    ) -> Result<()> {
        let ev = &mut ctx.accounts.event;
        ev.capacity = capacity;
        ev.sold = 0;

        let tickets = &mut ctx.accounts.tickets;
        tickets.count = 0;

        let soldout = &mut ctx.accounts.soldout;
        soldout.flag = false;
        Ok(())
    }

    pub fn sell_tickets(
        ctx: Context<SellTickets>,
        to_sell: u32,
    ) -> Result<()> {
        let ev = &mut ctx.accounts.event;
        let available = ev.capacity.saturating_sub(ev.sold);
        let actual = to_sell.min(available);

        // ループでチケットを売却
        for _ in 0..actual {
            ev.sold += 1;
            ctx.accounts.tickets.count += 1;
        }
        ctx.accounts.soldout.flag = ev.sold == ev.capacity;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEvent<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4)]
    pub event: Account<'info, EventData>,
    #[account(init, payer = user, space = 8 + 4)]
    pub tickets: Account<'info, TicketData>,
    #[account(init, payer = user, space = 8 + 1)]
    pub soldout: Account<'info, SoldOutData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SellTickets<'info> {
    #[account(mut)] pub event: Account<'info, EventData>,
    #[account(mut)] pub tickets: Account<'info, TicketData>,
    #[account(mut)] pub soldout: Account<'info, SoldOutData>,
}

#[account]
pub struct EventData {
    pub capacity: u32,
    pub sold: u32,
}

#[account]
pub struct TicketData {
    pub count: u32,
}

#[account]
pub struct SoldOutData {
    pub flag: bool,
}
