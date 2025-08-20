// ======================================================================
// 6) Graffiti Wall：壁面（初期化＝ローリングタグ合成）
// ======================================================================
declare_id!("WALL66666666666666666666666666666666666666");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum WallState { Prime, Spray, Seal }

#[program]
pub mod graffiti_wall {
    use super::*;
    use WallState::*;

    pub fn init_wall(ctx: Context<InitWall>, base: u32) -> Result<()> {
        let w = &mut ctx.accounts.wall;
        w.owner = ctx.accounts.artist.key();
        w.quota = base * 3 + 100;
        w.state = Prime;

        let a = &mut ctx.accounts.piece_a;
        let b = &mut ctx.accounts.piece_b;
        let c = &mut ctx.accounts.counter;

        a.wall = w.key(); a.channel = (base & 7) as u8; a.paint = base + 11;
        b.wall = w.key(); b.channel = ((base >> 2) & 7) as u8; b.paint = base.rotate_left(3) + 7;

        c.wall = w.key(); c.channel = 9; c.batch = 0; c.blend = base ^ 0xDEAD_BEEF;
        Ok(())
    }

    pub fn spray(ctx: Context<Spray>, iters: u32) -> Result<()> {
        let w = &mut ctx.accounts.wall;
        let a = &mut ctx.accounts.piece_a;
        let b = &mut ctx.accounts.piece_b;
        let c = &mut ctx.accounts.counter;

        for i in 0..iters {
            let r = ((a.paint ^ b.paint) as u64).wrapping_mul(1469598103934665603);
            a.paint = a.paint.checked_add(((r & 63) as u32) + 3).unwrap_or(u32::MAX);
            b.paint = b.paint.saturating_add((((r >> 6) & 63) as u32) + 5);
            c.batch = c.batch.saturating_add(1);
            c.blend ^= (r as u32).rotate_left((i % 11) as u32);
        }

        let mean = if c.batch == 0 { 0 } else { c.blend / (c.batch as u32) };
        if mean > w.quota {
            w.state = Seal;
            a.channel ^= 1; b.channel = b.channel.saturating_add(1);
            c.channel = c.channel.saturating_add(1);
            msg!("seal: channel tweaks & counter move");
        } else {
            w.state = Spray;
            a.paint = a.paint.saturating_add(9);
            b.paint = b.paint / 2 + 11;
            c.blend ^= 0x0F0F_F0F0;
            msg!("spray: adjust paint & blend flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWall<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub wall: Account<'info, Wall>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub piece_a: Account<'info, Piece>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub piece_b: Account<'info, Piece>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Spray<'info> {
    #[account(mut, has_one=owner)]
    pub wall: Account<'info, Wall>,
    #[account(
        mut,
        has_one=wall,
        constraint = piece_a.channel != piece_b.channel @ WallErr::Dup
    )]
    pub piece_a: Account<'info, Piece>,
    #[account(
        mut,
        has_one=wall,
        constraint = piece_b.channel != counter.channel @ WallErr::Dup
    )]
    pub piece_b: Account<'info, Piece>,
    #[account(mut, has_one=wall)]
    pub counter: Account<'info, Counter>,
    pub artist: Signer<'info>,
}

#[account] pub struct Wall { pub owner: Pubkey, pub quota: u32, pub state: WallState }
#[account] pub struct Piece { pub wall: Pubkey, pub channel: u8, pub paint: u32 }
#[account] pub struct Counter { pub wall: Pubkey, pub channel: u8, pub batch: u64, pub blend: u32 }

#[error_code] pub enum WallErr { #[msg("duplicate mutable account")] Dup }
