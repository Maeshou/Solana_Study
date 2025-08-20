use anchor_lang::prelude::*;

declare_id!("EnergyForge1111111111111111111111111111111111");

#[program]
pub mod energy_forge_confusion {
    use super::*;

    pub fn charge_energy(ctx: Context<ChargeEnergy>, charge_rate: u8) -> Result<()> {
        let metadata = PlayerMetadata::try_from_slice(&ctx.accounts.meta_account.data.borrow())?;
        let player = &mut ctx.accounts.energy_meter;
        let caller = ctx.accounts.charger.key();

        if metadata.player != caller {
            return Err(ProgramError::InvalidAccountData.into());
        }

        player.current_energy += charge_rate as u32 * 10;
        player.charge_events += 1;
        player.last_charge_slot = Clock::get()?.slot;

        for i in 0..3 {
            let ghost = Pubkey::new_unique();
            if ghost != caller {
                player.observed_sources.push(ghost);
            }
        }

        if player.current_energy > player.max_capacity {
            player.overflow_log.push((Clock::get()?.unix_timestamp, player.current_energy));
            player.current_energy = player.max_capacity;
        }

        player.audit_log.push(format!("{} charged at slot {}", caller, player.last_charge_slot));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ChargeEnergy<'info> {
    pub charger: Signer<'info>,
    /// CHECK: Could be any data-structured account
    pub meta_account: AccountInfo<'info>,
    #[account(mut)]
    pub energy_meter: Account<'info, EnergyMeter>,
}

#[account]
pub struct EnergyMeter {
    pub current_energy: u32,
    pub max_capacity: u32,
    pub charge_events: u32,
    pub last_charge_slot: u64,
    pub observed_sources: Vec<Pubkey>,
    pub overflow_log: Vec<(i64, u32)>,
    pub audit_log: Vec<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PlayerMetadata {
    pub player: Pubkey,
    pub class_id: u8,
}
