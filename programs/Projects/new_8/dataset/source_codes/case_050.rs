use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
declare_id!("GaLleryCuratorGGGG8888888888888888888888");

#[program]
pub mod gallery_curator_g {
    use super::*;

    pub fn create_gallery(ctx: Context<CreateGallery>, rooms: u16) -> Result<()> {
        let g = &mut ctx.accounts.gallery;
        g.owner = ctx.accounts.curator.key();
        g.rooms = rooms % 60 + 6;
        g.exhibits = 7;
        g.visits = 3;
        Ok(())
    }

    pub fn rotate(ctx: Context<Rotate>, count: u16, user_bump: u8) -> Result<()> {
        let g = &mut ctx.accounts.gallery;

        // 1) while（長め）
        let mut turn = 2u32;
        while turn < (count as u32 % 21 + 6) {
            g.exhibits = g.exhibits.saturating_add(turn);
            if g.exhibits % 5 != 2 { g.visits = g.visits.saturating_add(1); }
            let pack = (turn * 3) % 10 + 1;
            g.rooms = g.rooms.saturating_add(pack as u16);
            turn = turn.saturating_add(4);
        }

        // 2) PDA検証
        let seeds = &[b"ticket_pad", ctx.accounts.curator.key.as_ref(), &[user_bump]];
        let p = Pubkey::create_program_address(seeds, ctx.program_id).map_err(|_| error!(GalErr::SeedIssue))?;
        if p != ctx.accounts.ticket_pad.key() { return Err(error!(GalErr::TicketKey)); }

        // 3) if（長め）
        if count > 10 {
            let add = (count % 7) as u32 + 2;
            g.visits = g.visits.saturating_add(add);
            let stamp = g.owner.to_bytes()[1];
            g.exhibits = g.exhibits.saturating_add(stamp as u32);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateGallery<'info> {
    #[account(init, payer = curator, space = 8 + 32 + 2 + 4 + 4,
        seeds=[b"gallery", curator.key().as_ref()], bump)]
    pub gallery: Account<'info, Gallery>,
    #[account(mut)]
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Rotate<'info> {
    #[account(mut, seeds=[b"gallery", curator.key().as_ref()], bump)]
    pub gallery: Account<'info, Gallery>,
    /// CHECK
    pub ticket_pad: AccountInfo<'info>,
    pub curator: Signer<'info>,
}
#[account] pub struct Gallery { pub owner: Pubkey, pub rooms: u16, pub exhibits: u32, pub visits: u32 }
#[error_code] pub enum GalErr { #[msg("seed issue")] SeedIssue, #[msg("ticket key mismatch")] TicketKey }
