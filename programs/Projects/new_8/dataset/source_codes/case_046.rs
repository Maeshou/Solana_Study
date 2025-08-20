use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
declare_id!("Art1factMixerBBBB333333333333333333333333");

#[program]
pub mod artifact_mixer_b {
    use super::*;

    pub fn create_mixer(ctx: Context<CreateMixer>, slots: u16) -> Result<()> {
        let m = &mut ctx.accounts.mixer;
        m.owner = ctx.accounts.crafter.key();
        m.slots = slots % 40 + 8;
        m.heat = 12;
        m.batches = 4;
        Ok(())
    }

    pub fn blend(ctx: Context<Blend>, level: u8, user_bump: u8) -> Result<()> {
        let m = &mut ctx.accounts.mixer;

        // 1) while（長め）
        let mut step = 1u32;
        while step < ((level as u32 % 18) + 5) {
            m.heat = m.heat.saturating_add(step);
            let temp = (m.heat % 9) + 2;
            m.batches = m.batches.saturating_add(temp);
            if m.batches % 4 != 1 { m.slots = m.slots.saturating_add(1); }
            step = step.saturating_add(3);
        }

        // 2) PDA検証
        let seeds = &[b"coolant_cell", ctx.accounts.crafter.key.as_ref(), &[user_bump]];
        let addr = Pubkey::create_program_address(seeds, ctx.program_id).map_err(|_| error!(MixErr::SeedBad))?;
        if addr != ctx.accounts.coolant_cell.key() { return Err(error!(MixErr::CoolantKey)); }

        // 3) if（長め）
        if level > 100 {
            let mut buf = [0u8; 4];
            buf[0] = level;
            m.heat = m.heat.saturating_add(buf[0] as u32);
            m.batches = m.batches.saturating_add((buf[0] as u32 % 5) + 1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMixer<'info> {
    #[account(init, payer = crafter, space = 8 + 32 + 2 + 4 + 4,
        seeds=[b"mixer", crafter.key().as_ref()], bump)]
    pub mixer: Account<'info, Mixer>,
    #[account(mut)]
    pub crafter: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Blend<'info> {
    #[account(mut, seeds=[b"mixer", crafter.key().as_ref()], bump)]
    pub mixer: Account<'info, Mixer>,
    /// CHECK
    pub coolant_cell: AccountInfo<'info>,
    pub crafter: Signer<'info>,
}
#[account] pub struct Mixer { pub owner: Pubkey, pub slots: u16, pub heat: u32, pub batches: u32 }
#[error_code] pub enum MixErr { #[msg("seed invalid")] SeedBad, #[msg("coolant key mismatch")] CoolantKey }
