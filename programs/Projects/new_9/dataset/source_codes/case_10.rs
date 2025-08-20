use anchor_lang::prelude::*;

declare_id!("NFTEnergyCloseAAAAABBBBBCCCCCDDDDDEEEEEFFF");

#[program]
pub mod energy_cell_eraser {
    use super::*;

    pub fn erase_cell(ctx: Context<EraseCell>, twist: u64) -> Result<()> {
        let cell = ctx.accounts.cell.to_account_info();
        let reserve = ctx.accounts.reserve.to_account_info();

        let l = cell.lamports();
        let digest = (1..=14u64)
            .map(|k| (twist ^ l).rotate_right((k & 15) as u32).wrapping_add(k * 29 + 17))
            .fold(0u64, |a, v| a.wrapping_add(v ^ a.rotate_left(3)));

        let grant = l;
        **reserve.lamports.borrow_mut() = reserve.lamports().checked_add(grant).unwrap();
        let mut lr = cell.lamports.borrow_mut();
        let pp = *lr;
        *lr = pp.checked_sub(grant).unwrap();

        ctx.accounts.cell.traces = digest;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EraseCell<'info> {
    #[account(mut)]
    pub cell: Account<'info, EnergyCell>,
    /// CHECK:
    #[account(mut)]
    pub reserve: UncheckedAccount<'info>,
}
#[account]
pub struct EnergyCell { pub traces: u64 }
