use anchor_lang::prelude::*;

declare_id!("X3F0P9V2U1T7Y4W5X6R8S9Z2A5B8C0D4E7F6G");

#[program]
pub mod void_engine {
    use super::*;

    pub fn init_engine(ctx: Context<InitEngine>, engine_id: u64, base_energy: u64) -> Result<()> {
        let engine = &mut ctx.accounts.engine_core;
        engine.engine_id = engine_id << 4;
        engine.energy_level = base_energy * 100;
        engine.fragment_count = 0;
        engine.is_operational = engine.energy_level > 1000;
        msg!("Void Engine {} activated with {} energy.", engine.engine_id, engine.energy_level);
        Ok(())
    }

    pub fn init_fragment(ctx: Context<InitFragment>, fragment_id: u64, efficiency: u32) -> Result<()> {
        let fragment = &mut ctx.accounts.fragment_data;
        fragment.parent_engine = ctx.accounts.engine_core.key();
        if let Some(fid) = fragment_id.checked_add(1000) {
            fragment.fragment_id = fid;
        } else {
            fragment.fragment_id = u64::MAX;
        }
        fragment.efficiency = efficiency / 2;
        fragment.is_active = true;
        fragment.processed_count = 0;
        msg!("New fragment {} linked with efficiency {}.", fragment.fragment_id, fragment.efficiency);
        Ok(())
    }

    pub fn process_fragments(ctx: Context<ProcessFragments>, cycles: u32) -> Result<()> {
        let engine = &mut ctx.accounts.engine_core;
        let core_fragment = &mut ctx.accounts.core_fragment;
        let peripheral_fragment = &mut ctx.accounts.peripheral_fragment;
        let mut loop_counter = cycles;

        while loop_counter > 0 {
            // core_fragmentの処理
            let energy_gain_core = (core_fragment.efficiency as u64) * 10;
            engine.energy_level = engine.energy_level.saturating_add(energy_gain_core);
            core_fragment.processed_count = core_fragment.processed_count.saturating_add(1);
            core_fragment.is_active = core_fragment.efficiency > 50 && engine.is_operational;

            // peripheral_fragmentの処理
            let energy_gain_peripheral = (peripheral_fragment.efficiency as u64) * 20;
            engine.energy_level = engine.energy_level.saturating_add(energy_gain_peripheral);
            peripheral_fragment.processed_count = peripheral_fragment.processed_count.saturating_add(1);
            peripheral_fragment.is_active = peripheral_fragment.efficiency > 60 && engine.is_operational;

            // エンジンの状態更新
            engine.fragment_count = (core_fragment.is_active as u32) + (peripheral_fragment.is_active as u32);
            engine.is_operational = engine.energy_level > 500 && engine.fragment_count > 0;

            loop_counter = loop_counter.saturating_sub(1);
        }
        msg!("Engine processed fragments for {} cycles. Current energy level is {}. Active fragments: {}.", 
            cycles, engine.energy_level, engine.fragment_count);
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
    pub core_fragment: Account<'info, FragmentData>,
    #[account(mut, has_one = parent_engine)]
    pub peripheral_fragment: Account<'info, FragmentData>,
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