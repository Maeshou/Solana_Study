use anchor_lang::prelude::*;

declare_id!("Iota9999999999999999999999999999999999999");

#[program]
pub mod iota_example {
    use super::*;

    pub fn init_iota(ctx: Context<InitIota>, d1: u8, d2: u32, d3: u64) -> Result<()> {
        let acc = &mut ctx.accounts.iota_account;
        acc.d1 = d1;
        acc.d2 = d2;
        acc.d3 = d3;
        acc.bump = *ctx.bumps.get("iota_account").unwrap();
        Ok(())
    }

    pub fn update_iota(ctx: Context<UpdateIota>) -> Result<()> {
        let acc = &mut ctx.accounts.iota_account;

        // ループ + 分岐①
        for i in 0..(acc.d2 as usize + 1) {
            if i % 2 == 0 {
                msg!("even i: {}", i);
                acc.d2 = acc.d2.checked_add(i as u32).unwrap();
                acc.d1 = acc.d1.checked_add(1).unwrap();
                msg!("d1 & d2 updated");
            } else {
                msg!("odd i: {}", i);
                acc.d3 = acc.d3.checked_mul(2).unwrap();
                msg!("d3 doubled");
            }
        }

        // 分岐②
        if acc.d3 > 1000 {
            msg!("d3 big");
            acc.d1 = acc.d1.checked_sub(1).unwrap();
            acc.d2 = acc.d2.checked_mul(3).unwrap();
        } else {
            msg!("d3 small");
            acc.d3 = acc.d3.checked_add(acc.d2 as u64).unwrap();
            msg!("d3 increased");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitIota<'info> {
    #[account(init, seeds = [b"iota"], bump, payer = payer, space = 8 + 1 + 4 + 8 + 1)]
    pub iota_account: Account<'info, IotaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateIota<'info> {
    #[account(mut, seeds = [b"iota"], bump)]
    pub iota_account: Account<'info, IotaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct IotaAccount {
    pub d1: u8,
    pub d2: u32,
    pub d3: u64,
    pub bump: u8,
}
