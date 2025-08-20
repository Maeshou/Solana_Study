use anchor_lang::prelude::*;

declare_id!("X3F0P9V2U1T7Y4W5X6R8S9Z2A5B8C0D4E7F6G");

#[program]
pub mod void_engine {
    use super::*;

    pub fn init_engine(ctx: Context<InitEngine>, engine_id: u64, base_energy: u64) -> Result<()> {
        let engine = &mut ctx.accounts.engine_core;
        engine.engine_id = engine_id.rotate_left(4);
        engine.energy_level = base_energy.checked_mul(100).unwrap_or(u64::MAX);
        engine.fragment_count = 0;
        engine.is_operational = engine.energy_level > 1000;
        msg!("Void Engine {} activated with {} energy.", engine.engine_id, engine.energy_level);
        Ok(())
    }

    pub fn init_fragment(ctx: Context<InitFragment>, fragment_id: u64, efficiency: u32) -> Result<()> {
        let fragment = &mut ctx.accounts.fragment_data;
        fragment.parent_engine = ctx.accounts.engine_core.key();
        fragment.fragment_id = fragment_id.checked_add(1000).unwrap_or(u64::MAX);
        fragment.efficiency = efficiency.checked_div(2).unwrap_or(1);
        fragment.is_active = true;
        fragment.processed_count = 0;
        msg!("New fragment {} linked with efficiency {}.", fragment.fragment_id, fragment.efficiency);
        Ok(())
    }

    pub fn process_fragments(ctx: Context<ProcessFragments>, cycles: u32) -> Result<()> {
        let engine = &mut ctx.accounts.engine_core;
        let fragment1 = &mut ctx.accounts.fragment1;
        let fragment2 = &mut ctx.accounts.fragment2;
        let mut loop_counter = cycles;

        while loop_counter > 0 {
            // fragment1の処理
            let energy_gain1 = (fragment1.efficiency as u64).checked_mul(10).unwrap_or(u64::MAX);
            engine.energy_level = engine.energy_level.checked_add(energy_gain1).unwrap_or(u64::MAX);
            fragment1.processed_count = fragment1.processed_count.checked_add(1).unwrap_or(u32::MAX);
            fragment1.is_active = fragment1.efficiency > 50 && engine.is_operational;

            // fragment2の処理
            let energy_gain2 = (fragment2.efficiency as u64).checked_mul(20).unwrap_or(u64::MAX);
            engine.energy_level = engine.energy_level.checked_add(energy_gain2).unwrap_or(u64::MAX);
            fragment2.processed_count = fragment2.processed_count.checked_add(1).unwrap_or(u32::MAX);
            fragment2.is_active = fragment2.efficiency > 60 && engine.is_operational;

            // エンジンの状態更新
            engine.is_operational = engine.energy_level > 500 && (fragment1.is_active || fragment2.is_active);

            loop_counter = loop_counter.checked_sub(1).unwrap_or(0);
        }
        msg!("Engine processed fragments for {} cycles. Current energy level is {}.", cycles, engine.energy_level);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(engine_id: u64, base_energy: u64)]
pub struct InitEngine<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 8 + 4 + 1)]
    pub engine_core: Account<'info, EngineCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(fragment_id: u64, efficiency: u32)]
pub struct InitFragment<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 4 + 1 + 4)]
    pub fragment_data: Account<'info, FragmentData>,
    #[account(mut)]
    pub engine_core: Account<'info, EngineCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(cycles: u32)]
pub struct ProcessFragments<'info> {
    #[account(mut)]
    pub engine_core: Account<'info, EngineCore>,
    #[account(mut, has_one = parent_engine)]
    pub fragment1: Account<'info, FragmentData>,
    #[account(mut, has_one = parent_engine)]
    pub fragment2: Account<'info, FragmentData>,
    pub signer: Signer<'info>,
}

#[account]
pub struct EngineCore {
    engine_id: u64,
    energy_level: u64,
    fragment_count: u32,
    is_operational: bool,
}

#[account]
pub struct FragmentData {
    parent_engine: Pubkey,
    fragment_id: u64,
    efficiency: u32,
    is_active: bool,
    processed_count: u32,
}