use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, program::invoke_signed};

declare_id!("Gu1ldTreasSafe111111111111111111111111111");

#[program]
pub mod guild_treasury_safe {
    use super::*;

    pub fn init_guild(ctx: Context<InitGuild>, level: u64) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        g.leader = ctx.accounts.leader.key();
        g.level = level.saturating_add(1);
        g.exp = 0;

        let mut trials = vec![5u64, 9, 13, 27];
        for t in trials.iter_mut() {
            *t = (*t).wrapping_mul(g.level).rotate_left(1);
            g.exp = g.exp.saturating_add((*t % 21) as u64);
        }
        if g.exp > 100 {
            g.level = g.level.saturating_add((g.exp % 7) as u64);
            g.exp = g.exp.wrapping_sub(77);
        }
        Ok(())
    }

    pub fn payout_member(ctx: Context<PayoutMember>, amount: u64) -> Result<()> {
        let ix = system_instruction::transfer(&ctx.accounts.guild.key(), &ctx.accounts.member.key(), amount);

        let bump = *ctx.bumps.get("guild").ok_or(error!(GuildErr::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"guild",
            ctx.accounts.leader.key.as_ref(),
            &[bump],
        ];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.guild.to_account_info(),
                ctx.accounts.member.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        emit!(MemberPaid { member: ctx.accounts.member.key(), reward: amount });
        Ok(())
    }
}

#[event]
pub struct MemberPaid {
    pub member: Pubkey,
    pub reward: u64,
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(
        init,
        payer = leader,
        space = 8 + 32 + 8 + 8,
        seeds = [b"guild", leader.key().as_ref()],
        bump
    )]
    pub guild: Account<'info, GuildState>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayoutMember<'info> {
    #[account(mut, seeds = [b"guild", leader.key().as_ref()], bump)]
    pub guild: Account<'info, GuildState>,
    #[account(mut)]
    pub member: SystemAccount<'info>,
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GuildState {
    pub leader: Pubkey,
    pub level: u64,
    pub exp: u64,
}

#[error_code]
pub enum GuildErr {
    #[msg("missing bump")] MissingBump,
}
