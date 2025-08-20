use anchor_lang::prelude::*;

declare_id!("Delta4444444444444444444444444444444444444");

#[program]
pub mod delta_example {
    use super::*;

    pub fn init_delta(ctx: Context<InitDelta>, x: u8, y: u32, z: u64) -> Result<()> {
        let acc = &mut ctx.accounts.delta_account;
        acc.x = x;
        acc.y = y;
        acc.z = z;
        acc.bump = *ctx.bumps.get("delta_account").unwrap();
        Ok(())
    }

    pub fn update_delta(ctx: Context<UpdateDelta>) -> Result<()> {
        let acc = &mut ctx.accounts.delta_account;

        // ループ + 分岐①
        let mut i = 0;
        while i < (acc.x as usize + 3) {
            if i % 4 == 0 {
                msg!("idx {} divisible by 4", i);
                acc.x = acc.x.checked_add(2).unwrap();
                acc.y = acc.y.checked_add(acc.x as u32).unwrap();
                msg!("x and y updated");
            } else {
                msg!("idx {} not divisible by 4", i);
                acc.z = acc.z.checked_mul(2).unwrap();
                msg!("z doubled");
            }
            i += 1;
        }

        // 分岐②
        if acc.y < 50 {
            msg!("y is small: {}", acc.y);
            acc.z = acc.z.checked_sub(acc.y as u64).unwrap();
            acc.y = acc.y.checked_add(20).unwrap();
        } else {
            msg!("y is large: {}", acc.y);
            acc.x = acc.x.checked_sub(1).unwrap();
            msg!("x decremented");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDelta<'info> {
    #[account(init, seeds = [b"delta"], bump, payer = payer, space = 8 + 1 + 4 + 8 + 1)]
    pub delta_account: Account<'info, DeltaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateDelta<'info> {
    #[account(mut, seeds = [b"delta"], bump)]
    pub delta_account: Account<'info, DeltaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct DeltaAccount {
    pub x: u8,
    pub y: u32,
    pub z: u64,
    pub bump: u8,
}
