use anchor_lang::prelude::*;

declare_id!("En3rgy16161616161616161616161616161616161616");

#[program]
pub mod energy_flow {
    use super::*;

    pub fn init_grid(ctx: Context<InitGrid>) -> Result<()> {
        let g = &mut ctx.accounts.grid;
        g.node = ctx.accounts.installer.key();
        g.buffer = [0u16; 6];
        g.load = 0;
        g.mode = 0;
        Ok(())
    }

    pub fn act_cycle(ctx: Context<CycleEnergy>, pulse: u16) -> Result<()> {
        let g = &mut ctx.accounts.grid;
        let updater = &ctx.accounts.updater;

        for i in 0..6 {
            let v = (pulse ^ (i as u16).rotate_left(2)) & 0xFF;
            g.buffer[i] = g.buffer[i].wrapping_add(v);
        }

        if g.buffer.iter().all(|&x| x > 1000) {
            g.load = g.buffer.iter().sum::<u16>() % 2048;
            g.mode = g.load.trailing_zeros() as u8;
        }

        g.node = updater.key(); // Type Cosplay
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGrid<'info> {
    #[account(init, payer = installer, space = 8 + 32 + 12 + 2 + 1)]
    pub grid: Account<'info, EnergyGrid>,
    #[account(mut)]
    pub installer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CycleEnergy<'info> {
    #[account(mut)]
    pub grid: Account<'info, EnergyGrid>,
    /// CHECK: ノード管理者とエネルギー更新者を区別しない
    pub updater: AccountInfo<'info>,
}

#[account]
pub struct EnergyGrid {
    pub node: Pubkey,
    pub buffer: [u16; 6],
    pub load: u16,
    pub mode: u8,
}
