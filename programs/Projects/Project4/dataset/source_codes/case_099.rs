use anchor_lang::prelude::*;

declare_id!("KappaAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod kappa_example {
    use super::*;

    pub fn init_kappa(ctx: Context<InitKappa>, x1: u8, x2: u32, x3: u64) -> Result<()> {
        let acc = &mut ctx.accounts.kappa_account;
        acc.x1 = x1;
        acc.x2 = x2;
        acc.x3 = x3;
        acc.bump = *ctx.bumps.get("kappa_account").unwrap();
        Ok(())
    }

    pub fn update_kappa(ctx: Context<UpdateKappa>) -> Result<()> {
        let acc = &mut ctx.accounts.kappa_account;

        // ループ + 分岐①
        for i in 0..4 {
            if i < 2 {
                msg!("i < 2: {}", i);
                acc.x1 = acc.x1.checked_add(1).unwrap();
                acc.x2 = acc.x2.checked_add(acc.x1 as u32).unwrap();
                msg!("x1 & x2 incremented");
            } else {
                msg!("i >= 2: {}", i);
                acc.x3 = acc.x3.checked_mul((i + 1) as u64).unwrap();
                msg!("x3 updated");
            }
        }

        // 分岐②
        if acc.x2 % 5 == 0 {
            msg!("x2 % 5 == 0");
            acc.x3 = acc.x3.checked_sub(acc.x2 as u64).unwrap();
            acc.x1 = acc.x1.checked_add(2).unwrap();
        } else {
            msg!("x2 % 5 != 0");
            acc.x2 = acc.x2.checked_mul(2).unwrap();
            msg!("x2 doubled");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitKappa<'info> {
    #[account(init, seeds = [b"kappa"], bump, payer = payer, space = 8 + 1 + 4 + 8 + 1)]
    pub kappa_account: Account<'info, KappaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateKappa<'info> {
    #[account(mut, seeds = [b"kappa"], bump)]
    pub kappa_account: Account<'info, KappaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct KappaAccount {
    pub x1: u8,
    pub x2: u32,
    pub x3: u64,
    pub bump: u8,
}
