use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("TiCkEtBoOk00000000000000000000000000002");

#[program]
pub mod ticket_book {
    use super::*;

    pub fn issue_ticket(ctx: Context<IssueTicket>, route: Vec<u8>, qty: u16, bump: u8) -> Result<()> {
        // 経路の整形とダイジェスト
        let mut r = route.clone();
        if r.len() > 40 { r.truncate(40); }
        let mut digest: u64 = 5381;
        for c in r.iter() { digest = (digest.wrapping_shl(5)).wrapping_add(digest) ^ (*c as u64); }

        // 入力 bump 使用（該当点）
        let seeds = [&ctx.accounts.issuer.key().to_bytes()[..], &r[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(TicketErr::KeyMismatch))?;
        if addr != ctx.accounts.ticket_cell.key() {
            return Err(error!(TicketErr::KeyMismatch));
        }

        // チケット発行処理：限度と在庫
        let b = &mut ctx.accounts.book;
        let mut add = qty;
        if add > 500 { add = 500; }
        b.issuer = ctx.accounts.issuer.key();
        b.route = r;
        b.issued = b.issued.saturating_add(add as u32);
        b.hash = b.hash.wrapping_add(digest);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct IssueTicket<'info> {
    #[account(mut)]
    pub book: Account<'info, Book>,
    /// CHECK:
    pub ticket_cell: AccountInfo<'info>,
    pub issuer: AccountInfo<'info>,
}

#[account]
pub struct Book {
    pub issuer: Pubkey,
    pub route: Vec<u8>,
    pub issued: u32,
    pub hash: u64,
}

#[error_code]
pub enum TicketErr {
    #[msg("Ticket cell PDA mismatch")]
    KeyMismatch,
}
