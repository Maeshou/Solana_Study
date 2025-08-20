use anchor_lang::prelude::*;

declare_id!("Beta2222222222222222222222222222222222222");

#[program]
pub mod beta_example {
    use super::*;

    pub fn init_beta(ctx: Context<InitBeta>, count: u8, amount: u32, total: u64) -> Result<()> {
        let acc = &mut ctx.accounts.beta_account;
        acc.count = count;
        acc.amount = amount;
        acc.total = total;
        acc.bump = *ctx.bumps.get("beta_account").unwrap();
        Ok(())
    }

    pub fn update_beta(ctx: Context<UpdateBeta>) -> Result<()> {
        let acc = &mut ctx.accounts.beta_account;

        // ループ + 分岐①
        for i in 0..(acc.amount as usize) {
            if i % 3 == 0 {
                msg!("multiple of 3: {}", i);
                acc.amount = acc.amount.checked_add(5).unwrap();
                acc.count = acc.count.checked_add(2).unwrap();
                msg!("count now {}", acc.count);
            } else {
                msg!("not mult of 3: {}", i);
                let new_total = acc.total.checked_add(i as u64).unwrap();
                acc.total = new_total;
                msg!("total now {}", acc.total);
            }
        }

        // 分岐②
        if acc.total > 500 {
            msg!("total exceeds 500");
            acc.amount = acc.amount.checked_mul(3).unwrap();
            acc.count = acc.count.checked_sub(1).unwrap();
            msg!("amount x3, count -1");
        } else {
            msg!("total within limit");
            acc.total = acc.total.checked_add(acc.amount as u64).unwrap();
            msg!("total increased");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBeta<'info> {
    #[account(init, seeds = [b"beta"], bump, payer = payer, space = 8 + 1 + 4 + 8 + 1)]
    pub beta_account: Account<'info, BetaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateBeta<'info> {
    #[account(mut, seeds = [b"beta"], bump)]
    pub beta_account: Account<'info, BetaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct BetaAccount {
    pub count: u8,
    pub amount: u32,
    pub total: u64,
    pub bump: u8,
}
