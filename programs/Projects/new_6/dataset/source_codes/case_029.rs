use anchor_lang::prelude::*;

declare_id!("EnerDistSys0000000000000000000000000000000");

#[program]
pub mod energy_distribution {
    use super::*;

    pub fn distribute(ctx: Context<DistributeEnergy>, amount: u16) -> Result<()> {
        let distributor = &mut ctx.accounts.distributor;
        let consumer = &mut ctx.accounts.consumer;

        // distributor と consumer が同じアカウントでも制限なし
        consumer.energy += amount as u32;
        distributor.total_given = distributor.total_given.saturating_add(amount as u32);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DistributeEnergy<'info> {
    #[account(mut)]
    pub distributor: Account<'info, EnergyCenter>,
    #[account(mut)]
    pub consumer: Account<'info, UserDevice>,
}

#[account]
pub struct EnergyCenter {
    pub total_given: u32,
}

#[account]
pub struct UserDevice {
    pub energy: u32,
}
