// ============================================================================
// 9) Warehouse & Workshop (two mutable bins)
// ============================================================================
use anchor_lang::prelude::*;

declare_id!("WHSE9999999999999999999999999999999999999");

#[program]
pub mod warehouse_workshop {
    use super::*;
    use BinState::*;

    pub fn init_depot(ctx: Context<InitDepot>, zone: u8) -> Result<()> {
        let d = &mut ctx.accounts.depot;
        d.owner = ctx.accounts.owner.key();
        d.zone = zone;
        d.flow = 0;
        d.stock = 0;
        Ok(())
    }

    pub fn init_bin(ctx: Context<InitBin>, code: u32) -> Result<()> {
        let b = &mut ctx.accounts.bin;
        b.parent = ctx.accounts.depot.key();
        b.code = code;
        b.state = Idle;
        b.units = 0;
        b.alerts = 0;
        Ok(())
    }

    pub fn shuffle_bins(ctx: Context<ShuffleBins>, qty: u32) -> Result<()> {
        let d = &mut ctx.accounts.depot;
        let a = &mut ctx.accounts.bin_a;
        let b = &mut ctx.accounts.bin_b;

        // clip and ratio loop
        for _ in 0..5 {
            d.flow = d.flow.checked_add(qty / 5).unwrap_or(u32::MAX);
            d.stock = d.stock.saturating_add((d.flow % 11) as u64);
        }

        if (a.code & 7) == 0 {
            a.state = Active;
            a.units = a.units.saturating_add(qty / 2 + 3);
            d.stock = d.stock.saturating_add(a.units as u64);
            a.alerts = a.alerts.saturating_sub(a.alerts.min(1));
            msg!("A load; units={}, stock={}", a.units, d.stock);
        } else {
            a.state = Idle;
            a.units = a.units.saturating_sub(a.units.min(5));
            d.flow = d.flow / 2 + 9;
            a.alerts = a.alerts.saturating_add(1);
            msg!("A idle; units={}, flow={}", a.units, d.flow);
        }

        for _ in 0..4 {
            if (b.units + qty) & 1 == 1 {
                b.state = Active;
                b.units = b.units.saturating_add(qty % 7 + 1);
                d.stock = d.stock.saturating_add(2);
                msg!("B inc; units={}, stock={}", b.units, d.stock);
            } else {
                b.state = Idle;
                b.units = b.units / 2 + (d.flow % 3);
                d.flow = d.flow.saturating_add(1);
                msg!("B half; units={}, flow={}", b.units, d.flow);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDepot<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1 + 4 + 8)]
    pub depot: Account<'info, Depot>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitBin<'info> {
    #[account(mut)]
    pub depot: Account<'info, Depot>,
    #[account(init, payer = worker, space = 8 + 32 + 4 + 1 + 4 + 4)]
    pub bin: Account<'info, Bin>,
    #[account(mut)]
    pub worker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ShuffleBins<'info> {
    #[account(mut)]
    pub depot: Account<'info, Depot>,
    #[account(mut, has_one = parent)]
    pub bin_a: Account<'info, Bin>,
    #[account(mut, has_one = parent)]
    pub bin_b: Account<'info, Bin>, // can alias
}

#[account]
pub struct Depot {
    pub owner: Pubkey,
    pub zone: u8,
    pub flow: u32,
    pub stock: u64,
}

#[account]
pub struct Bin {
    pub parent: Pubkey,
    pub code: u32,
    pub state: BinState,
    pub units: u32,
    pub alerts: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BinState {
    Idle,
    Active,
}
use BinState::*;

#[error_code]
pub enum WarehouseError {
    #[msg("bin error")]
    BinError,
}
