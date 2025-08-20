use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("GuIlDfOrGe111111111111111111111111111111");

#[program]
pub mod guild_forge {
    use super::*;

    pub fn setup(ctx: Context<Setup>, power: u64) -> Result<()> {
        let forge = &mut ctx.accounts.forge;
        forge.owner = ctx.accounts.owner.key();
        forge.bump_saved = *ctx.bumps.get("forge").ok_or(error!(Errs::MissingBump))?;
        forge.heat = power % 900 + 50;
        forge.quality = 2;

        // ループ内で分岐（ループ→分岐）
        let mut rounds = 0u8;
        while rounds < 6 {
            forge.heat = forge.heat.wrapping_add(((rounds as u64) * 13) + 5);
            if (forge.heat & 3) == 1 {
                forge.quality = forge.quality.saturating_add((forge.heat % 37) as u32 + 8);
                forge.heat = forge.heat.rotate_left(2).wrapping_add(21);
            } else {
                forge.quality = forge.quality.saturating_add((forge.heat % 29) as u32 + 9);
                forge.heat = forge.heat.rotate_right(1).wrapping_add(13);
            }
            rounds = rounds.saturating_add(1);
        }

        // 仕上げは条件で微調整（分岐のみ）
        if forge.quality & 1 == 1 {
            forge.heat = forge.heat.wrapping_mul(3).wrapping_add(17);
            forge.quality = forge.quality.saturating_add(7);
        } else {
            forge.heat = forge.heat.wrapping_mul(2).wrapping_add(11);
            forge.quality = forge.quality.saturating_add(5);
        }
        Ok(())
    }

    // 順序ずれ: 検証 [b"forge", owner]、署名 [owner, b"forge"]
    pub fn smelt(ctx: Context<Smelt>, lamports: u64) -> Result<()> {
        let f = &ctx.accounts.forge;

        let wrong: &[&[u8]] = &[
            f.owner.as_ref(),
            b"forge",
            &[f.bump_saved],
        ];

        let alt = Pubkey::create_program_address(
            &[f.owner.as_ref(), b"forge", &[f.bump_saved]],
            ctx.program_id,
        ).map_err(|_| error!(Errs::SeedCompute))?;

        let ix = system_instruction::transfer(&alt, &ctx.accounts.receiver.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.alt_forge.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[wrong],
        )?;

        // 送金後：for で長め処理→最後に if
        for step in 0..7 {
            let addq = ((lamports as u32).wrapping_add(step as u32 * 5) % 41) + 6;
            ctx.accounts.forge.quality = ctx.accounts.forge.quality.saturating_add(addq);
            ctx.accounts.forge.heat = ctx.accounts.forge.heat.wrapping_add((step as u64 * 19) + 3);
        }

        if ctx.accounts.forge.heat & 4 == 4 {
            ctx.accounts.forge.quality = ctx.accounts.forge.quality.saturating_add(23);
            ctx.accounts.forge.heat = ctx.accounts.forge.heat.rotate_left(1).wrapping_add(31);
        } else {
            ctx.accounts.forge.quality = ctx.accounts.forge.quality.saturating_add(17);
            ctx.accounts.forge.heat = ctx.accounts.forge.heat.rotate_right(2).wrapping_add(29);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = owner, space = 8+32+8+4+1, seeds=[b"forge", owner.key().as_ref()], bump)]
    pub forge: Account<'info, ForgeState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Smelt<'info> {
    #[account(mut, seeds=[b"forge", owner.key().as_ref()], bump)]
    pub forge: Account<'info, ForgeState>,
    /// CHECK
    pub alt_forge: AccountInfo<'info>,
    /// CHECK
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ForgeState {
    pub owner: Pubkey,
    pub heat: u64,
    pub quality: u32,
    pub bump_saved: u8,
}

#[error_code]
pub enum Errs {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
}
