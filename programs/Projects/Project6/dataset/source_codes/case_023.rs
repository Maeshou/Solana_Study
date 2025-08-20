use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("Gui1dBAdg3333333333333333333333333333333");

#[program]
pub mod guild_badge {
    use super::*;
    use Tier::*;

    pub fn init_guild(ctx: Context<InitGuild>, name: String) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        g.authority = ctx.accounts.owner.key();
        g.name = name;
        g.total = 0;
        Ok(())
    }

    pub fn init_member(ctx: Context<InitMember>, tier: Tier) -> Result<()> {
        let m = &mut ctx.accounts.member;
        m.parent = ctx.accounts.guild.key();
        m.tier = tier;
        m.score = 0;
        Ok(())
    }

    pub fn award(ctx: Context<Award>, bonus: u16) -> Result<()> {
        require!(
            ctx.accounts.badge_ta.mint == ctx.accounts.badge_mint.key(),
            BadgeErr::MintMismatch
        );
        require!(
            ctx.accounts.badge_ta.owner == ctx.accounts.owner.key(),
            BadgeErr::OwnerMismatch
        );

        let g = &mut ctx.accounts.guild;
        let a = &mut ctx.accounts.member_a;
        let b = &mut ctx.accounts.member_b;
        let r = &mut ctx.accounts.report;

        let mut acc = 0u32;
        for i in 0..5 {
            let inc = ((bonus as u32) ^ (i as u32 * 13)) & 0x3FF;
            r.hist[i] = r.hist[i].saturating_add(inc);
            acc = acc.saturating_add(inc);
        }

        if acc & 1 == 0 {
            a.score = a.score.saturating_add((acc / 5) as u64);
            g.total = g.total.saturating_add(a.score / 10);
            r.ok += 1;
        } else {
            b.score = b.score.saturating_add((acc / 4) as u64);
            g.total = g.total.saturating_add(b.score / 12);
            r.ng += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 64 + 8)]
    pub guild: Account<'info, Guild>,
    #[account(init, payer = owner, space = 8 + 4*5 + 4 + 4)]
    pub report: Account<'info, Report>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitMember<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(init, payer = owner, space = 8 + 32 + 1 + 8)]
    pub member: Account<'info, Member>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Award<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(
        mut,
        has_one = parent,
        constraint = member_a.tier as u8 != member_b.tier as u8 @ BadgeErr::CosplayBlocked
    )]
    pub member_a: Account<'info, Member>,
    #[account(mut, has_one = parent)]
    pub member_b: Account<'info, Member>,
    #[account(mut)]
    pub report: Account<'info, Report>,

    pub badge_mint: Account<'info, Mint>,
    #[account(mut)]
    pub badge_ta: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Guild {
    pub authority: Pubkey,
    pub name: String,
    pub total: u64,
}

#[account]
pub struct Member {
    pub parent: Pubkey,
    pub tier: Tier,
    pub score: u64,
}

#[account]
pub struct Report {
    pub hist: [u32; 5],
    pub ok: u32,
    pub ng: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Bronze,
    Silver,
    Gold,
}

#[error_code]
pub enum BadgeErr {
    #[msg("type cosplay blocked")] CosplayBlocked,
    #[msg("mint mismatch")] MintMismatch,
    #[msg("owner mismatch")] OwnerMismatch,
}
