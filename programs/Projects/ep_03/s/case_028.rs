use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRafflSvc01");

#[program]
pub mod raffle_service {
    use super::*;

    /// ラッフルチケットを購入し、売上金をプールに積み上げるが、
    /// raffle_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn buy_raffle_ticket(ctx: Context<BuyRaffleTicket>) -> Result<()> {
        let raffle = &mut ctx.accounts.raffle_account;
        let ticket = &mut ctx.accounts.ticket_account;
        let price = raffle.ticket_price;

        // 1. ユーザーから購入料金を徴収（所有者チェックなし）
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= price;
        **ctx.accounts.prize_pool.to_account_info().lamports.borrow_mut() += price;

        // 2. 販売済チケット数をインクリメント
        raffle.tickets_sold = raffle.tickets_sold.checked_add(1).unwrap();

        // 3. ユーザーのチケット保有数を更新
        ticket.tickets = ticket.tickets.checked_add(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuyRaffleTicket<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付与して照合チェックを行うべき
    pub raffle_account: Account<'info, Raffle>,

    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付与して照合チェックを行うべき
    pub ticket_account: Account<'info, TicketAccount>,

    /// チケット購入者（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 賞金プール（Lamports 保管先）
    #[account(mut)]
    pub prize_pool: AccountInfo<'info>,
}

#[account]
pub struct Raffle {
    /// 本来このラッフルを管理するユーザーの Pubkey
    pub owner: Pubkey,
    /// チケット1枚あたりの価格（Lamports）
    pub ticket_price: u64,
    /// これまでに販売したチケット数
    pub tickets_sold: u64,
}

#[account]
pub struct TicketAccount {
    /// 本来このチケット口座を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// ユーザーが保有するチケット枚数
    pub tickets: u64,
}
