use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("SaVeBumpMixer222222222222222222222222222");

#[program]
pub mod save_bump_mixer {
    use super::*;

    pub fn init_mixer(ctx: Context<InitMixer>, slots: u16) -> Result<()> {
        let mixer = &mut ctx.accounts.mixer;
        mixer.admin = ctx.accounts.operator.key();
        mixer.slots = slots % 40 + 8;
        mixer.intensity = 11;
        mixer.batches = 2;

        let bump_val = *ctx.bumps.get("mixer").ok_or(error!(MixErr::MissingBump))?;
        mixer.saved_bump = bump_val;

        // 長めの初期演算
        let mut warmup = (mixer.slots as u32 % 9) + 2;
        while warmup != 0 {
            mixer.intensity = mixer.intensity.saturating_add(warmup);
            warmup = warmup.saturating_sub(1);
        }
        if mixer.intensity % 4 != 1 {
            mixer.batches = mixer.batches.saturating_add(3);
        }
        Ok(())
    }

    // 不安全: saved_bump を別シード "coolant_pool" に適用
    pub fn blend(ctx: Context<Blend>, level: u8) -> Result<()> {
        let mixer = &mut ctx.accounts.mixer;

        let seeds = &[b"coolant_pool", mixer.admin.as_ref(), &[mixer.saved_bump]];
        let target = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(MixErr::SeedCompute))?;
        if target != ctx.accounts.coolant_pool.key() {
            return Err(error!(MixErr::PoolMismatch));
        }

        // 長めの処理
        let mut cursor = 1u32;
        while cursor < ((level as u32 % 21) + 6) {
            mixer.intensity = mixer.intensity.saturating_add(cursor);
            let gain = (mixer.intensity % 7) + 2;
            mixer.batches = mixer.batches.saturating_add(gain);
            cursor = cursor.saturating_add(3);
        }
        if level > 100 {
            let pad = level as u32 % 11 + 2;
            mixer.intensity = mixer.intensity.saturating_add(pad);
            let sig = mixer.admin.to_bytes()[0];
            mixer.batches = mixer.batches.saturating_add((sig % 5) as u32 + 1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMixer<'info> {
    #[account(
        init, payer = operator, space = 8 + 32 + 2 + 4 + 4 + 1,
        seeds=[b"mixer", operator.key().as_ref()], bump
    )]
    pub mixer: Account<'info, Mixer>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Blend<'info> {
    #[account(
        mut,
        seeds=[b"mixer", operator.key().as_ref()], bump
    )]
    pub mixer: Account<'info, Mixer>,
    /// CHECK: 手動導出に依存
    pub coolant_pool: AccountInfo<'info>,
    pub operator: Signer<'info>,
}

#[account]
pub struct Mixer {
    pub admin: Pubkey,
    pub slots: u16,
    pub intensity: u32,
    pub batches: u32,
    pub saved_bump: u8, // ← 保存後に別シードへ流用（危険）
}

#[error_code]
pub enum MixErr {
    #[msg("seed compute failed")] SeedCompute,
    #[msg("pool key mismatch")] PoolMismatch,
    #[msg("missing bump in context")] MissingBump,
}
