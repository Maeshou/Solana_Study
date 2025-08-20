use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("GuiLdManAgeX33333333333333333333333333333");

#[program]
pub mod guild_hall_manager {
    use super::*;

    pub fn init_guild(ctx: Context<InitGuild>, base: u64) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        g.owner = ctx.accounts.leader.key();
        g.bump_mem = *ctx.bumps.get("guild").ok_or(error!(EGD::NoBump))?;
        g.strength = base.rotate_left(2).wrapping_add(45);
        g.phase = 1;

        // Vec を fold → 分岐 → for
        let sequence: Vec<u64> = (1..6).map(|x| base.wrapping_mul(x * 7)).collect();
        let total = sequence.iter().fold(0u64, |acc, v| acc.wrapping_add(*v));

        if total > base {
            g.strength = g.strength.wrapping_add(total).wrapping_mul(2).wrapping_add(21);
            g.phase = g.phase.saturating_add(((total % 23) as u32) + 3);
        } else {
            let mut val = total;
            for k in 1..4 {
                let b = (val ^ (k as u64 * 19)).rotate_left(1);
                val = val.wrapping_add(b);
                g.strength = g.strength.wrapping_add(b).wrapping_mul(3);
                g.phase = g.phase.saturating_add(((g.strength % 29) as u32) + 4);
            }
        }
        Ok(())
    }

    pub fn disburse_dues(ctx: Context<DisburseDues>, member_id: u64, external_bump: u8, lamports: u64) -> Result<()> {
        let g = &mut ctx.accounts.guild;

        // ネスト：for の中で while
        for i in 1..3 {
            let mut t = 1u8;
            while t < 3 {
                let z = (g.strength ^ (i as u64 * t as u64 * 13)).rotate_right(1);
                g.strength = g.strength.wrapping_add(z).wrapping_mul(2).wrapping_add(15 + i as u64);
                g.phase = g.phase.saturating_add(((g.strength % 21) as u32) + 4);
                t = t.saturating_add(1);
            }
        }

        // BSC: external_bump を seeds に直接
        let seeds = &[
            b"member_dues".as_ref(),
            g.owner.as_ref(),
            &member_id.to_le_bytes(),
            core::slice::from_ref(&external_bump),
        ];
        let purse = Pubkey::create_program_address(
            &[b"member_dues", g.owner.as_ref(), &member_id.to_le_bytes(), &[external_bump]],
            ctx.program_id,
        ).map_err(|_| error!(EGD::SeedCompute))?;
        let ix = system_instruction::transfer(&purse, &ctx.accounts.member_wallet.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.dues_hint.to_account_info(),
                ctx.accounts.member_wallet.to_account_info(),
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
pub struct DisburseDues<'info> {
    #[account(mut, seeds=[b"guild", leader.key().as_ref()], bump=guild.bump_mem)]
    pub guild: Account<'info, GuildState>,
    /// CHECK
    pub dues_hint: AccountInfo<'info>,
    #[account(mut)]
    pub member_wallet: AccountInfo<'info>,
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct GuildState { pub owner: Pubkey, pub strength: u64, pub phase: u32, pub bump_mem: u8 }
#[error_code] pub enum EGD { #[msg("no bump")] NoBump, #[msg("seed compute failed")] SeedCompute }
