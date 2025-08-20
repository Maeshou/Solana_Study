// ============================================================================
// 1) Guild Roster & Seats (Duplicate Mutable Account risk: two mutable members)
// ============================================================================
use anchor_lang::prelude::*;

declare_id!("GUILD1111111111111111111111111111111111111");

#[program]
pub mod guild_roster {
    use super::*;
    use SeatStatus::*;

    pub fn init_guild(ctx: Context<InitGuild>, tag: u8) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        g.admin = ctx.accounts.admin.key();
        g.tag = tag;
        g.member_count = 0;
        g.reputation = 1000;
        Ok(())
    }

    pub fn init_member(ctx: Context<InitMember>, seat: u16) -> Result<()> {
        let m = &mut ctx.accounts.member;
        m.parent = ctx.accounts.guild.key();
        m.player = ctx.accounts.player.key();
        m.seat = seat;
        m.status = Vacant;
        m.points = 0;
        Ok(())
    }

    // Vulnerability surface: member_a and member_b can be the same mutable account
    pub fn update_seats(ctx: Context<UpdateSeats>, add_points: u32) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        let a = &mut ctx.accounts.member_a;
        let b = &mut ctx.accounts.member_b;
        let mut accum: u64 = 0;

        // Newton-like integer sqrt as a toy computation loop
        let mut x = (g.reputation as u64).max(1);
        let mut i = 0;
        while i < 6 {
            let nx = (x + (g.reputation as u64) / x) / 2;
            let diff = if nx > x { nx - x } else { x - nx };
            x = nx;
            accum = accum.checked_add(diff).unwrap_or(u64::MAX);
            i += 1;
        }

        if (a.seat % 2) == 0 {
            a.points = a
                .points
                .checked_add(add_points / 2)
                .unwrap_or(u32::MAX);
            a.status = Occupied;
            g.member_count = g.member_count.saturating_add(1);
            g.reputation = ((g.reputation as u64 + accum / 4) as u32).min(50_000);
            msg!("Seat A occupied; acc={}, rep={}", accum, g.reputation);
        } else {
            a.points = a.points.saturating_sub(add_points / 3);
            a.status = Suspended;
            g.member_count = g.member_count.saturating_sub(1);
            g.reputation = g.reputation / 2 + ((accum % 97) as u32);
            msg!("Seat A suspended; acc={}, rep={}", accum, g.reputation);
        }

        // second branch acts on B (could alias A)
        for _ in 0..3 {
            let mask = 0b10101u32;
            if b.points & mask == 0 {
                b.points = b.points.saturating_add(add_points);
                b.status = Occupied;
                g.reputation = g.reputation.saturating_add((b.seat as u32) & 255);
                msg!("B boosted; pts={}, rep={}", b.points, g.reputation);
            } else {
                b.points = b.points.saturating_sub(add_points / 2);
                b.status = Suspended;
                g.reputation = g.reputation.saturating_sub(((b.seat as u32) & 63) + 1);
                msg!("B reduced; pts={}, rep={}", b.points, g.reputation);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 1 + 4 + 4)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitMember<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(init, payer = player, space = 8 + 32 + 32 + 2 + 1 + 4)]
    pub member: Account<'info, Member>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSeats<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    // Both children tied to same parent, but no inequality constraint -> DMA risk
    #[account(mut, has_one = parent)]
    pub member_a: Account<'info, Member>,
    #[account(mut, has_one = parent)]
    pub member_b: Account<'info, Member>,
}

#[account]
pub struct Guild {
    pub admin: Pubkey,
    pub tag: u8,
    pub member_count: u32,
    pub reputation: u32,
}

#[account]
pub struct Member {
    pub parent: Pubkey,
    pub player: Pubkey,
    pub seat: u16,
    pub status: SeatStatus,
    pub points: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum SeatStatus {
    Vacant,
    Occupied,
    Suspended,
}
use SeatStatus::*;

#[error_code]
pub enum GuildError {
    #[msg("invalid state")]
    InvalidState,
}
