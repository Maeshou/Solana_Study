use anchor_lang::prelude::*;

declare_id!("Gamma3333333333333333333333333333333333333");

#[program]
pub mod gamma_example {
    use super::*;

    pub fn init_gamma(ctx: Context<InitGamma>, a: u8, b: u32, c: u64) -> Result<()> {
        let acc = &mut ctx.accounts.gamma_account;
        acc.a = a;
        acc.b = b;
        acc.c = c;
        acc.bump = *ctx.bumps.get("gamma_account").unwrap();
        Ok(())
    }

    pub fn update_gamma(ctx: Context<UpdateGamma>) -> Result<()> {
        let acc = &mut ctx.accounts.gamma_account;

        // ループ + 分岐①
        for i in 1..(acc.b as usize + 1) {
            if i % 5 == 0 {
                msg!("hit multiple of 5: {}", i);
                acc.b = acc.b.checked_add(10).unwrap();
                acc.a = acc.a.checked_add(1).unwrap();
                msg!("a now {}", acc.a);
            } else {
                msg!("skip: {}", i);
                acc.c = acc.c.checked_sub(i as u64).unwrap();
                msg!("c now {}", acc.c);
            }
        }

        // 分岐②
        if acc.c % 2 == 0 {
            msg!("c is even");
            acc.b = acc.b.checked_mul(2).unwrap();
            acc.a = acc.a.checked_add(3).unwrap();
            msg!("b doubled, a +3");
        } else {
            msg!("c is odd");
            acc.c = acc.c.checked_add(acc.b as u64).unwrap();
            msg!("c + b");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGamma<'info> {
    #[account(init, seeds = [b"gamma"], bump, payer = payer, space = 8 + 1 + 4 + 8 + 1)]
    pub gamma_account: Account<'info, GammaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateGamma<'info> {
    #[account(mut, seeds = [b"gamma"], bump)]
    pub gamma_account: Account<'info, GammaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct GammaAccount {
    pub a: u8,
    pub b: u32,
    pub c: u64,
    pub bump: u8,
}
