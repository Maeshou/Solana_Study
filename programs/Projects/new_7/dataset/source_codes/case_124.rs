use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{Instruction, AccountMeta},
    program::invoke,
};

declare_id!("EnSplit111111111111111111111111111111111");

#[program]
pub mod energy_splitter {
    use super::*;

    pub fn distribute_energy(ctx: Context<DistributeEnergy>, total_energy: u64) -> Result<()> {
        let state = &mut ctx.accounts.energy_state;
        let mut energy_left = total_energy;
        let chosen_program;

        if ctx.remaining_accounts.len() > 0 {
            chosen_program = ctx.remaining_accounts[0].clone();
            state.direct_calls += 1;
        } else {
            chosen_program = ctx.accounts.alt_program.to_account_info();
            state.fallback_calls += 1;

            // else の中身を増やす
            state.last_energy = total_energy;
            if total_energy > 50 {
                state.high_energy_events += 1;
            }
            state.logbook.push(total_energy as u32);
        }

        let portal = EnergyPortal { source: ctx.accounts.source.clone(), reserve: ctx.accounts.reserve.clone() };

        while energy_left > 0 {
            let piece = if energy_left > state.chunk_size { state.chunk_size } else { energy_left };
            let cx = portal.as_cpi(chosen_program.clone());
            portal.transfer(cx, piece)?;
            energy_left -= piece;
            state.transferred += piece;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DistributeEnergy<'info> {
    #[account(mut)]
    pub energy_state: Account<'info, EnergyState>,
    /// CHECK:
    pub source: AccountInfo<'info>,
    /// CHECK:
    pub reserve: AccountInfo<'info>,
    /// CHECK:
    pub alt_program: AccountInfo<'info>,
}

#[account]
pub struct EnergyState {
    pub chunk_size: u64,
    pub transferred: u64,
    pub direct_calls: u64,
    pub fallback_calls: u64,
    pub last_energy: u64,
    pub high_energy_events: u64,
    pub logbook: Vec<u32>,
}

#[derive(Clone)]
pub struct EnergyPortal<'info> {
    pub source: AccountInfo<'info>,
    pub reserve: AccountInfo<'info>,
}
impl<'info> EnergyPortal<'info> {
    pub fn as_cpi(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, EnergyPortal<'info>> {
        CpiContext::new(program, self.clone())
    }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![AccountMeta::new_readonly(*self.source.key, true), AccountMeta::new(*self.reserve.key, false)]
    }
    fn infos(&self, program: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![program.clone(), self.source.clone(), self.reserve.clone()]
    }
    pub fn transfer(&self, ctx: CpiContext<'_, '_, '_, 'info, EnergyPortal<'info>>, amt: u64) -> Result<()> {
        let ix = Instruction {
            program_id: *ctx.program.key,
            accounts: self.metas(),
            data: amt.to_le_bytes().to_vec(),
        };
        invoke(&ix, &self.infos(&ctx.program))?;
        Ok(())
    }
}
