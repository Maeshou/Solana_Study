// (6) Market Kiosk — ミニマーケットの出品と検品
use anchor_lang::prelude::*;
declare_id!("MarKe7K1oSk6666666666666666666666666666666");

#[program]
pub mod market_kiosk {
    use super::*;
    use Actor::*;

    pub fn init_kiosk(ctx: Context<InitKiosk>) -> Result<()> {
        let k = &mut ctx.accounts.kiosk;
        k.owner = ctx.accounts.operator.key();
        k.fee_bps = 25;
        k.listed = 0;
        Ok(())
    }

    pub fn init_card(ctx: Context<InitCard>, actor: Actor) -> Result<()> {
        let c = &mut ctx.accounts.card;
        c.kiosk = ctx.accounts.kiosk.key();
        c.actor = actor;
        c.score = 0;
        c.buf = [0; 4];
        Ok(())
    }

    pub fn process_listing(ctx: Context<ProcessListing>, price: u64) -> Result<()> {
        let k = &mut ctx.accounts.kiosk;
        let s = &mut ctx.accounts.seller;
        let i = &mut ctx.accounts.inspector;
        let b = &mut ctx.accounts.board;

        for j in 0..4 {
            s.buf[j] = s.buf[j].saturating_add(((price >> (j * 8)) as u32) & 0xFF);
        }

        if s.actor == Seller {
            s.score = s.score.saturating_add(((price % 997) as u32) + k.fee_bps as u32);
            k.listed = k.listed.saturating_add(1);
            b.cursor = b.cursor.saturating_add(1);
            b.acc = b.acc.wrapping_add(price.rotate_left(9));
            msg!("Seller path");
        } else {
            i.score = i.score.saturating_add(((price >> 2) as u32) % 463);
            k.fee_bps = (k.fee_bps + 1).min(100);
            b.cursor = b.cursor.saturating_add(2);
            b.acc = b.acc ^ price.rotate_right(11);
            msg!("Inspector path");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitKiosk<'info> {
    #[account(init, payer = operator, space = 8 + Kiosk::MAX)]
    pub kiosk: Account<'info, Kiosk>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitCard<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub kiosk: Account<'info, Kiosk>,
    #[account(init, payer = user, space = 8 + Card::MAX)]
    pub card: Account<'info, Card>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ProcessListing<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub kiosk: Account<'info, Kiosk>,
    #[account(mut, has_one = kiosk, owner = crate::ID)]
    pub board: Account<'info, Board>,
    #[account(mut, has_one = kiosk, owner = crate::ID)]
    pub seller: Account<'info, Card>,
    #[account(
        mut,
        has_one = kiosk,
        owner = crate::ID,
        constraint = seller.actor != inspector.actor @ ErrCode::CosplayBlocked
    )]
    pub inspector: Account<'info, Card>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Kiosk { pub owner: Pubkey, pub fee_bps: u16, pub listed: u64 }
impl Kiosk { pub const MAX: usize = 32 + 2 + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Actor { Seller, Inspector, Broker }
use Actor::*;

#[account]
pub struct Card { pub kiosk: Pubkey, pub actor: Actor, pub score: u32, pub buf: [u32; 4] }
impl Card { pub const MAX: usize = 32 + 1 + 4 + 4 * 4; }

#[account]
pub struct Board { pub kiosk: Pubkey, pub acc: u64, pub cursor: u32 }
impl Board { pub const MAX: usize = 32 + 8 + 4; }

#[error_code]
pub enum ErrCode { #[msg("Type Cosplay blocked by actor mismatch")] CosplayBlocked }
