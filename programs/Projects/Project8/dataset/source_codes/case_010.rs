// Program 9: vendor_cashbox (System transferのみ; 複数回invokeでもseeds一貫)
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("VendorCashb0x999999999999999999999999999");

#[program]
pub mod vendor_cashbox {
    use super::*;

    pub fn init_box(ctx: Context<InitBox>, series: u64) -> Result<()> {
        let b = &mut ctx.accounts.cashbox;
        b.vendor = ctx.accounts.vendor.key();
        b.series = series.rotate_left(1).wrapping_add(5);
        b.turn = 0;
        Ok(())
    }

    pub fn disperse(ctx: Context<Disperse>, amount_each: u64, loops: u8) -> Result<()> {
        let bump = *ctx.bumps.get("cashbox").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"cashbox", ctx.accounts.vendor.key.as_ref(), &ctx.accounts.cashbox.series.to_le_bytes(), &[bump]];

        let mut i = 0u8;
        while i < loops {
            let val = amount_each.rotate_left((i % 4) as u32).wrapping_add(7);
            let ix = system_instruction::transfer(&ctx.accounts.cashbox.key(), &ctx.accounts.receiver.key(), val);
            invoke_signed(
                &ix,
                &[
                    ctx.accounts.cashbox.to_account_info(),
                    ctx.accounts.receiver.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBox<'info> {
    #[account(
        init,
        payer = vendor,
        space = 8 + 32 + 8 + 8,
        seeds=[b"cashbox", vendor.key().as_ref(), series.to_le_bytes().as_ref()],
        bump
    )]
    pub cashbox: Account<'info, CashBox>,
    #[account(mut)]
    pub vendor: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub series: u64,
}

#[derive(Accounts)]
pub struct Disperse<'info> {
    #[account(
        mut,
        seeds=[b"cashbox", vendor.key().as_ref(), cashbox.series.to_le_bytes().as_ref()],
        bump
    )]
    pub cashbox: Account<'info, CashBox>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
    pub vendor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CashBox {
    pub vendor: Pubkey,
    pub series: u64,
    pub turn: u64,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
