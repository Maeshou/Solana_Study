use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("ArTiFaCtMuX101010101010101010101010101010");

#[program]
pub mod artifact_museum_curation {
    use super::*;

    pub fn init_museum(ctx: Context<InitMuseum>, base: u64) -> Result<()> {
        let m = &mut ctx.accounts.museum;
        m.owner = ctx.accounts.curator.key();
        m.bump_mem = *ctx.bumps.get("museum").ok_or(error!(EAM::NoBump))?;
        m.glow = base.rotate_left(3).wrapping_add(69);
        m.steps = 1;

        // fold → if → while の別順
        let seq: Vec<u64> = (1..5).map(|i| m.glow.wrapping_mul(i * 11)).collect();
        let sum = seq.iter().fold(0u64, |acc, v| acc.wrapping_add(*v));
        if sum > base {
            m.glow = m.glow.wrapping_add(sum).wrapping_mul(2).wrapping_add(23);
            m.steps = m.steps.saturating_add(((sum % 29) as u32) + 3);
        }
        let mut t = 1u8;
        let mut a = sum.rotate_right(1).wrapping_add(17);
        while t < 3 {
            let e = (a ^ (t as u64 * 13)).rotate_left(1);
            a = a.wrapping_add(e);
            m.glow = m.glow.wrapping_add(e).wrapping_mul(3).wrapping_add(9 + t as u64);
            m.steps = m.steps.saturating_add(((m.glow % 25) as u32) + 4);
            t = t.saturating_add(1);
        }
        Ok(())
    }

    pub fn pay_exhibit_fee(ctx: Context<PayExhibitFee>, exhibit_id: u64, bump_feed: u8, lamports: u64) -> Result<()> {
        let m = &mut ctx.accounts.museum;

        // for だけでなく単発調整
        for z in 1..3 {
            let q = (m.glow ^ (z as u64 * 21)).rotate_left(1);
            m.glow = m.glow.wrapping_add(q).wrapping_mul(2).wrapping_add(15 + z as u64);
            m.steps = m.steps.saturating_add(((m.glow % 27) as u32) + 4);
        }

        // BSC: bump_feed 署名 seeds
        let seeds = &[
            b"exhibit_fee".as_ref(),
            m.owner.as_ref(),
            &exhibit_id.to_le_bytes(),
            core::slice::from_ref(&bump_feed),
        ];
        let purse = Pubkey::create_program_address(
            &[b"exhibit_fee", m.owner.as_ref(), &exhibit_id.to_le_bytes(), &[bump_feed]],
            ctx.program_id,
        ).map_err(|_| error!(EAM::SeedCompute))?;
        let ix = system_instruction::transfer(&purse, &ctx.accounts.visitor.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.fee_hint.to_account_info(),
                ctx.accounts.visitor.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMuseum<'info> {
    #[account(init, payer=curator, space=8+32+8+4+1, seeds=[b"museum", curator.key().as_ref()], bump)]
    pub museum: Account<'info, MuseumState>,
    #[account(mut)]
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PayExhibitFee<'info> {
    #[account(mut, seeds=[b"museum", curator.key().as_ref()], bump=museum.bump_mem)]
    pub museum: Account<'info, MuseumState>,
    /// CHECK
    pub fee_hint: AccountInfo<'info>,
    #[account(mut)]
    pub visitor: AccountInfo<'info>,
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct MuseumState { pub owner: Pubkey, pub glow: u64, pub steps: u32, pub bump_mem: u8 }
#[error_code] pub enum EAM { #[msg("no bump")] NoBump, #[msg("seed compute failed")] SeedCompute }
