use anchor_lang::prelude::*;

declare_id!("Theta8888888888888888888888888888888888888");

#[program]
pub mod theta_example {
    use super::*;

    pub fn init_theta(ctx: Context<InitTheta>, f: u8, g: u32, h: u64) -> Result<()> {
        let acc = &mut ctx.accounts.theta_account;
        acc.f = f;
        acc.g = g;
        acc.h = h;
        acc.bump = *ctx.bumps.get("theta_account").unwrap();
        Ok(())
    }

    pub fn update_theta(ctx: Context<UpdateTheta>) -> Result<()> {
        let acc = &mut ctx.accounts.theta_account;

        // ループ + 分岐①
        for i in 0..3 {
            if i == 1 {
                msg!("i is one");
                acc.f = acc.f.checked_add(5).unwrap();
                acc.g = acc.g.checked_add(acc.f as u32).unwrap();
                msg!("f+5 & g+f");
            } else {
                msg!("i != 1");
                acc.h = acc.h.checked_mul((i + 1) as u64).unwrap();
                msg!("h updated");
            }
        }

        // 分岐②
        if acc.g % 2 == 0 {
            msg!("g even");
            acc.h = acc.h.checked_sub(acc.g as u64).unwrap();
            acc.f = acc.f.checked_add(2).unwrap();
        } else {
            msg!("g odd");
            acc.g = acc.g.checked_mul(2).unwrap();
            msg!("g doubled");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTheta<'info> {
    #[account(init, seeds = [b"theta"], bump, payer = payer, space = 8 + 1 + 4 + 8 + 1)]
    pub theta_account: Account<'info, ThetaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTheta<'info> {
    #[account(mut, seeds = [b"theta"], bump)]
    pub theta_account: Account<'info, ThetaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct ThetaAccount {
    pub f: u8,
    pub g: u32,
    pub h: u64,
    pub bump: u8,
}
