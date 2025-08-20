use anchor_lang::prelude::*;

declare_id!("EvtSale11111111111111111111111111111111111");

/// イベント販売設定
#[account]
pub struct EventSale {
    pub seller:       Pubkey, // チケット販売者（主催者）
    pub price:        u64,    // １枚あたりの価格（lamports）
    pub total_tickets: u64,   // 発行枚数
    pub sold:         u64,    // 売れた枚数
}

/// 購入・保有チケット情報
#[account]
pub struct TicketAccount {
    pub owner: Pubkey, // チケット所有者
    pub sale:  Pubkey, // 本来は EventSale.key() と一致すべき
    pub redeemed: bool,// 既に引換済みかどうか
}

/// チケット購入イベント
#[event]
pub struct TicketPurchased {
    pub sale:    Pubkey,
    pub buyer:   Pubkey,
    pub quantity: u64,
}

/// チケット引換イベント
#[event]
pub struct TicketRedeemed {
    pub ticket_account: Pubkey,
    pub redeemer:       Pubkey,
}

#[derive(Accounts)]
pub struct InitializeSale<'info> {
    #[account(init, payer = seller, space = 8 + 32 + 8 + 8 + 8)]
    pub sale:          Account<'info, EventSale>,
    #[account(mut)]
    pub seller:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseTickets<'info> {
    /// EventSale.seller == seller.key() は検証される
    #[account(mut, has_one = seller)]
    pub sale:          Account<'info, EventSale>,

    /// 新規に TicketAccount を初期化。sale フィールドへは sale.key() を書くだけで検証ナシ
    #[account(init, payer = buyer, space = 8 + 32 + 32 + 1)]
    pub ticket:        Account<'info, TicketAccount>,

    #[account(mut)]
    pub buyer:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemTicket<'info> {
    /// TicketAccount.owner == redeemer.key() は検証される
    #[account(mut, has_one = owner)]
    pub ticket:        Account<'info, TicketAccount>,

    /// 本来は ticket.sale == sale.key() の検証が必要だが、ここでは一切入っていない
    #[account(mut)]
    pub sale:          Account<'info, EventSale>,

    pub redeemer:      Signer<'info>,
}

#[program]
pub mod ticketing_vuln {
    use super::*;

    /// イベントチケット販売の設定
    pub fn initialize_sale(ctx: Context<InitializeSale>, price: u64, total: u64) -> Result<()> {
        let sale = &mut ctx.accounts.sale;
        sale.seller = ctx.accounts.seller.key();
        sale.price = price;
        sale.total_tickets = total;
        sale.sold = 0;
        Ok(())
    }

    /// チケット購入
    pub fn purchase(ctx: Context<PurchaseTickets>, quantity: u64) -> Result<()> {
        let sale = &mut ctx.accounts.sale;
        let ticket = &mut ctx.accounts.ticket;

        // 本来は選択した quantity が残枚数以内かチェックし、
        // 購入料の支払いも行う必要があるが省略。

        // 脆弱性ポイント：
        // ticket.sale = sale.key(); としているだけで、
        // 次の redeem で sale が本当にこの sale かの検証がない。
        ticket.owner = ctx.accounts.buyer.key();
        ticket.sale = sale.key();
        ticket.redeemed = false;

        sale.sold = sale.sold.checked_add(quantity).unwrap();
        emit!(TicketPurchased {
            sale:    sale.key(),
            buyer:   ticket.owner,
            quantity,
        });
        Ok(())
    }

    /// チケット引換
    pub fn redeem(ctx: Context<RedeemTicket>) -> Result<()> {
        let sale = &mut ctx.accounts.sale;
        let ticket = &mut ctx.accounts.ticket;

        // 本来は必須:
        // require_keys_eq!(
        //     ticket.sale,
        //     sale.key(),
        //     TicketError::SaleMismatch
        // );
        //
        // もしくは
        // #[account(address = ticket.sale)]
        // pub sale: Account<'info, EventSale>,

        // チェックがないため、攻撃者は別の sale を用意して渡すことで
        // 本来無関係なイベントのチケットを引換できてしまう。
        require!(!ticket.redeemed, TicketError::AlreadyRedeemed);
        ticket.redeemed = true;
        emit!(TicketRedeemed {
            ticket_account: ticket.key(),
            redeemer:       ticket.owner,
        });
        Ok(())
    }
}

#[error_code]
pub enum TicketError {
    #[msg("このチケットは既に引換済みです")]
    AlreadyRedeemed,
    #[msg("TicketAccount と EventSale が一致しません")]
    SaleMismatch,
}
