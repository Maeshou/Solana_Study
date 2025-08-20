use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("R3licVaulT44444444444444444444444444444");

#[program]
pub mod relic_vault {
    use super::*;

    pub fn open_vault(ctx: Context<OpenVault>, entropy: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.owner = ctx.accounts.explorer.key();
        v.power = entropy.rotate_right(2).wrapping_add(19);
        v.cells = 2;
        v.star = 1;

        // enumerate → chunks → loop
        for (i, val) in [8u64, 13, 21, 34].iter().enumerate() {
            v.power = v.power.wrapping_add(val.rotate_left((i + 1) as u32));
        }
        for ch in [3u64, 5, 9, 14, 23, 37].chunks(3) {
            let mut t = 0u64;
            for u in ch { t = t.wrapping_add(*u); }
            v.cells = v.cells.saturating_add(((t % 3) as u16) + 1);
        }
        let mut c = 1u8;
        loop {
            v.star = v.star.saturating_add(1);
            if c > 2 { break; }
            v.power = v.power.wrapping_add((c as u64 * 7).rotate_left(1));
            c = c.saturating_add(1);
        }
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, base: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;

        if v.power > 40 { v.cells = v.cells.saturating_add(1); }
        v.star = v.star.saturating_add((v.power % 2) as u8);

        let seeds: &[&[u8]] = &[
            b"vault",
            ctx.accounts.explorer.key.as_ref(),
            ctx.accounts.realm.key().as_ref(),
            &[ctx.bumps["vault"]],
        ];
        let amt = base.saturating_add((v.power % 101) + 5);
        let ix = system_instruction::transfer(&ctx.accounts.vault.key(), &ctx.accounts.bank.key(), amt);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.bank.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenVault<'info> {
    #[account(
        init,
        payer = explorer,
        space = 8 + 32 + 8 + 2 + 1,
        seeds = [b"vault", explorer.key().as_ref(), realm.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub explorer: Signer<'info>,
    /// CHECK
    pub realm: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"vault", explorer.key().as_ref(), realm.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub bank: SystemAccount<'info>,
    pub explorer: Signer<'info>,
    /// CHECK
    pub realm: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub power: u64,
    pub cells: u16,
    pub star: u8,
}
