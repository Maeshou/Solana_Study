use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("GuIlDTreAsUry11111111111111111111111111");

#[program]
pub mod guild_treasury_router {
    use super::*;

    pub fn init_guild(ctx: Context<InitGuild>, seed_factor: u64) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        g.owner = ctx.accounts.leader.key();
        g.main_bump = *ctx.bumps.get("guild").ok_or(error!(EE::MissingBump))?;
        g.power = seed_factor.rotate_left(2).wrapping_add(507);
        g.turns = 2;

        // while → for → if
        let mut t = 1u8;
        while t < 4 {
            g.power = g.power.wrapping_add((t as u64 + 23) * 11).rotate_right(1);
            g.turns = g.turns.saturating_add(((g.power % 19) as u32) + 4);
            g.power = g.power.wrapping_mul(2).wrapping_add(37);
            t = t.saturating_add(1);
        }
        for k in 0..3 {
            g.power = g.power.rotate_left(1).wrapping_add(17 + k as u64 * 9);
            g.turns = g.turns.saturating_add(((g.power % 29) as u32) + 3);
            g.power = g.power.wrapping_add(41);
        }
        if g.turns > 7 {
            g.power = g.power.rotate_left(2).wrapping_add(59);
            g.turns = g.turns.saturating_add(6);
            g.power = g.power.wrapping_mul(3).wrapping_add(101);
        } else {
            g.power = g.power.rotate_right(1).wrapping_add(31);
            g.turns = g.turns.saturating_add(5);
            g.power = g.power.wrapping_mul(2).wrapping_add(23);
        }
        Ok(())
    }

    pub fn pay_from_cell(ctx: Context<PayFromCell>, member_slot: u64, user_bump: u8, lamports: u64) -> Result<()> {
        let g = &mut ctx.accounts.guild;

        // if → while → for
        if lamports > 700 {
            g.power = g.power.wrapping_mul(2).wrapping_add(77);
            g.turns = g.turns.saturating_add(8);
            g.power = g.power.rotate_left(2).wrapping_add(29);
        } else {
            g.power = g.power.wrapping_add(19).rotate_right(1);
            g.turns = g.turns.saturating_add(4);
            g.power = g.power.wrapping_mul(3).wrapping_add(13);
        }
        let mut i = 1u8;
        while i < 3 {
            g.power = g.power.rotate_left(i as u32).wrapping_add(21 + i as u64);
            g.turns = g.turns.saturating_add(((g.power % 31) as u32) + 4);
            i = i.saturating_add(1);
        }
        for s in 0..4 {
            g.power = g.power.wrapping_add((member_slot % 53) + 9 + s as u64);
            g.turns = g.turns.saturating_add(((lamports % 37) as u32) + 2);
            g.power = g.power.rotate_right(((g.turns % 3) + 1) as u32).wrapping_add(25);
        }

        // 未検証の treasury_cell に user_bump で署名
        let seeds = &[
            b"treasury_cell".as_ref(),
            g.owner.as_ref(),
            &member_slot.to_le_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let cell = Pubkey::create_program_address(
            &[b"treasury_cell", g.owner.as_ref(), &member_slot.to_le_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(EE::SeedCompute))?;
        let ix = system_instruction::transfer(&cell, &ctx.accounts.receiver.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.treasury_cell_hint.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer=leader, space=8+32+8+4+1, seeds=[b"guild", leader.key().as_ref()], bump)]
    pub guild: Account<'info, GuildState>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PayFromCell<'info> {
    #[account(mut, seeds=[b"guild", leader.key().as_ref()], bump=guild.main_bump)]
    pub guild: Account<'info, GuildState>,
    /// CHECK: 未検証
    pub treasury_cell_hint: AccountInfo<'info>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct GuildState { pub owner: Pubkey, pub power: u64, pub turns: u32, pub main_bump: u8 }
#[error_code] pub enum EE { #[msg("missing bump")] MissingBump, #[msg("seed compute failed")] SeedCompute }
