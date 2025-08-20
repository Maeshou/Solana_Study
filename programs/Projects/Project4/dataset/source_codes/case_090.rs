use anchor_lang::prelude::*;

declare_id!("Alpha1111111111111111111111111111111111111");

#[program]
pub mod alpha_example {
    use super::*;

    pub fn init_alpha(ctx: Context<InitAlpha>, data1: u8, data2: u32, data3: u64) -> Result<()> {
        let acc = &mut ctx.accounts.alpha_account;
        acc.data1 = data1;
        acc.data2 = data2;
        acc.data3 = data3;
        acc.bump = *ctx.bumps.get("alpha_account").unwrap();
        Ok(())
    }

    pub fn update_alpha(ctx: Context<UpdateAlpha>) -> Result<()> {
        let acc = &mut ctx.accounts.alpha_account;

        // ループ + 分岐 その1
        for i in 0..(acc.data2 as usize) {
            if i % 2 == 0 {
                msg!("even idx: {}", i);
                acc.data2 = acc.data2.checked_add(i as u32).unwrap();
                acc.data1 = acc.data1.checked_add(1).unwrap();
                msg!("data1 now {}", acc.data1);
            } else {
                msg!("odd idx: {}", i);
                let prod = acc.data3.checked_mul(i as u64).unwrap();
                acc.data3 = prod;
                msg!("data3 now {}", acc.data3);
            }
        }

        // 分岐 その2
        if acc.data2 > 1000 {
            msg!("data2 is large: {}", acc.data2);
            acc.data3 = acc.data3.checked_sub(acc.data2 as u64).unwrap();
            acc.data2 = acc.data2.checked_mul(2).unwrap();
        } else {
            msg!("data2 is small: {}", acc.data2);
            acc.data1 = acc.data1.checked_sub(1).unwrap();
            msg!("data1 decremented to {}", acc.data1);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAlpha<'info> {
    #[account(init, seeds = [b"alpha"], bump, payer = payer, space = 8 + 1 + 4 + 8 + 1)]
    pub alpha_account: Account<'info, AlphaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAlpha<'info> {
    #[account(mut, seeds = [b"alpha"], bump)]
    pub alpha_account: Account<'info, AlphaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct AlphaAccount {
    pub data1: u8,
    pub data2: u32,
    pub data3: u64,
    pub bump: u8,
}
