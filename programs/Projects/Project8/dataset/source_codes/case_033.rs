use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("EnergyForgeSafe11111111111111111111111111");

#[program]
pub mod energy_forge_safe {
    use super::*;

    pub fn init_forge(ctx: Context<InitForge>, core: u64) -> Result<()> {
        let f = &mut ctx.accounts.forge;
        f.master = ctx.accounts.master.key();
        f.core = core.rotate_left(1).wrapping_add(21);
        f.stage = 2;

        let mut v = f.core.rotate_right(1).wrapping_add(11);
        for _ in 0..3 {
            v = v.rotate_left(1).wrapping_mul(3).wrapping_add(7);
            f.stage = f.stage.saturating_add(((v % 17) as u32) + 1);
        }
        Ok(())
    }

    pub fn siphon(ctx: Context<Siphon>, lamports: u64) -> Result<()> {
        let ix = system_instruction::transfer(&ctx.accounts.forge.key(), &ctx.accounts.receiver.key(), lamports);

        let bump = *ctx.bumps.get("forge").ok_or(error!(ForgeErr::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"forge",
            ctx.accounts.master.key.as_ref(),
            &ctx.accounts.forge.core.to_le_bytes(),
            &[bump],
        ];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.forge.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        emit!(EnergyMoved { to: ctx.accounts.receiver.key(), amount: lamports });
        Ok(())
    }
}

#[event]
pub struct EnergyMoved {
    pub to: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct InitForge<'info> {
    #[account(
        init,
        payer = master,
        space = 8 + 32 + 8 + 4,
        seeds = [b"forge", master.key().as_ref(), core.to_le_bytes().as_ref()],
        bump
    )]
    pub forge: Account<'info, ForgeState>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub core: u64,
}

#[derive(Accounts)]
pub struct Siphon<'info> {
    #[account(
        mut,
        seeds = [b"forge", master.key().as_ref(), forge.core.to_le_bytes().as_ref()],
        bump
    )]
    pub forge: Account<'info, ForgeState>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ForgeState {
    pub master: Pubkey,
    pub core: u64,
    pub stage: u32,
}

#[error_code]
pub enum ForgeErr {
    #[msg("missing bump")] MissingBump,
}
