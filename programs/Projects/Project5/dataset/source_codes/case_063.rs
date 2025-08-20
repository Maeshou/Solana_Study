// ======================================================================
// 1) Guild management: guild roster + hall board
// ======================================================================
use anchor_lang::prelude::*;

declare_id!("GUILD111111111111111111111111111111111111111");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum GuildPhase { Forming, March, Rest }

#[program]
pub mod guild_roster_hall {
    use super::*;
    use GuildPhase::*;

    pub fn init_guild(ctx: Context<InitGuild>, cap: u32) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        let a = &mut ctx.accounts.member_alpha;
        let b = &mut ctx.accounts.member_beta;
        let h = &mut ctx.accounts.hall;

        g.owner = ctx.accounts.owner.key();
        g.cap = cap;
        g.phase = Forming;

        a.parent = g.key();
        a.seat = 1;
        a.power = 10;

        b.parent = g.key();
        b.seat = 2;
        b.power = 12;

        h.parent = g.key();
        h.channel = 9;
        h.flags = 0;
        h.notice = 0;

        Ok(())
    }

    pub fn update_march(ctx: Context<UpdateMarch>, steps: u32) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        let a = &mut ctx.accounts.member_alpha;
        let b = &mut ctx.accounts.member_beta;
        let h = &mut ctx.accounts.hall;

        for i in 0..steps {
            let da = (3 + (i % 5)) as u32;
            let db = (5 + (i % 7)) as u32;
            a.power = a.power.checked_add(da).unwrap_or(u32::MAX);
            b.power = b.power.checked_add(db).unwrap_or(u32::MAX);
            let mix = ((a.power as u64 + b.power as u64) / 6) as u32;
            h.notice = h.notice.rotate_left((i % 8) as u32) ^ mix;
        }

        let total = a.power as u64 + b.power as u64;
        if total > g.cap as u64 {
            g.phase = Rest;
            h.flags = h.flags.saturating_add(2);
            a.power = (a.power / 2) + 11;
            b.power = (b.power / 2) + 13;
            msg!("phase=Rest flags+2 damp powers total={}", total);
        } else {
            g.phase = March;
            h.notice = h.notice.saturating_add(7);
            a.power ^= 0x00FF_00FF;
            b.power = b.power.saturating_add(9);
            msg!("phase=March notice+7 xor/boost total={}", total);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub guild: Account<'info, GuildHead>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub member_alpha: Account<'info, GuildMember>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub member_beta: Account<'info, GuildMember>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4 + 4)]
    pub hall: Account<'info, GuildHall>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMarch<'info> {
    #[account(mut, has_one=owner)]
    pub guild: Account<'info, GuildHead>,
    #[account(
        mut,
        has_one=guild,
        constraint = member_alpha.seat != member_beta.seat @ GuildErr::DupAcc
    )]
    pub member_alpha: Account<'info, GuildMember>,
    #[account(
        mut,
        has_one=guild,
        constraint = member_beta.seat != hall.channel @ GuildErr::DupAcc
    )]
    pub member_beta: Account<'info, GuildMember>,
    #[account(mut, has_one=guild)]
    pub hall: Account<'info, GuildHall>,
    pub owner: Signer<'info>,
}

#[account]
pub struct GuildHead {
    pub owner: Pubkey,
    pub cap: u32,
    pub phase: GuildPhase,
}

#[account]
pub struct GuildMember {
    pub guild: Pubkey, // alias kept as parent in attributes
    pub seat: u8,
    pub power: u32,
}
impl GuildMember { pub fn parent(&self) -> Pubkey { self.guild } }

#[account]
pub struct GuildHall {
    pub parent: Pubkey,
    pub channel: u8,
    pub flags: u32,
    pub notice: u32,
}

#[error_code]
pub enum GuildErr {
    #[msg("duplicate mutable account")]
    DupAcc,
}