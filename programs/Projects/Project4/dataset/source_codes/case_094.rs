use anchor_lang::prelude::*;

declare_id!("Epsilon55555555555555555555555555555555555");

#[program]
pub mod epsilon_example {
    use super::*;

    pub fn init_epsilon(ctx: Context<InitEpsilon>, p: u8, q: u32, r: u64) -> Result<()> {
        let acc = &mut ctx.accounts.epsilon_account;
        acc.p = p;
        acc.q = q;
        acc.r = r;
        acc.bump = *ctx.bumps.get("epsilon_account").unwrap();
        Ok(())
    }

    pub fn update_epsilon(ctx: Context<UpdateEpsilon>) -> Result<()> {
        let acc = &mut ctx.accounts.epsilon_account;

        // ループ + 分岐①
        for i in 0..(acc.q as usize) {
            if i % 2 == 1 {
                msg!("odd loop: {}", i);
                acc.q = acc.q.checked_add(2).unwrap();
                acc.p = acc.p.checked_add(3).unwrap();
                msg!("p and q updated");
            } else {
                msg!("even loop: {}", i);
                acc.r = acc.r.checked_sub(i as u64).unwrap();
                msg!("r decreased");
            }
        }

        // 分岐②
        if acc.r > 200 {
            msg!("r > 200");
            acc.q = acc.q.checked_mul(2).unwrap();
            acc.p = acc.p.checked_sub(1).unwrap();
        } else {
            msg!("r <= 200");
            acc.r = acc.r.checked_add(acc.q as u64).unwrap();
            msg!("r increased");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEpsilon<'info> {
    #[account(init, seeds = [b"epsilon"], bump, payer = payer, space = 8 + 1 + 4 + 8 + 1)]
    pub epsilon_account: Account<'info, EpsilonAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateEpsilon<'info> {
    #[account(mut, seeds = [b"epsilon"], bump)]
    pub epsilon_account: Account<'info, EpsilonAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct EpsilonAccount {
    pub p: u8,
    pub q: u32,
    pub r: u64,
    pub bump: u8,
}
