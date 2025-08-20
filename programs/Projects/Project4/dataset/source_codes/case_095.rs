use anchor_lang::prelude::*;

declare_id!("Zeta6666666666666666666666666666666666666");

#[program]
pub mod zeta_example {
    use super::*;

    pub fn init_zeta(ctx: Context<InitZeta>, m: u8, n: u32, o: u64) -> Result<()> {
        let acc = &mut ctx.accounts.zeta_account;
        acc.m = m;
        acc.n = n;
        acc.o = o;
        acc.bump = *ctx.bumps.get("zeta_account").unwrap();
        Ok(())
    }

    pub fn update_zeta(ctx: Context<UpdateZeta>) -> Result<()> {
        let acc = &mut ctx.accounts.zeta_account;

        // ループ + 分岐①
        for i in 0..(acc.n as usize + 2) {
            if i % 6 == 0 {
                msg!("divisible by 6: {}", i);
                acc.n = acc.n.checked_add(1).unwrap();
                acc.m = acc.m.checked_add(2).unwrap();
                msg!("m & n incremented");
            } else {
                msg!("not divisible by 6: {}", i);
                acc.o = acc.o.checked_mul(3).unwrap();
                msg!("o tripled");
            }
        }

        // 分岐②
        if acc.m > 10 {
            msg!("m large");
            acc.o = acc.o.checked_sub(acc.m as u64).unwrap();
            acc.n = acc.n.checked_mul(2).unwrap();
        } else {
            msg!("m small");
            acc.m = acc.m.checked_add(5).unwrap();
            msg!("m +5");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitZeta<'info> {
    #[account(init, seeds = [b"zeta"], bump, payer = payer, space = 8 + 1 + 4 + 8 + 1)]
    pub zeta_account: Account<'info, ZetaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateZeta<'info> {
    #[account(mut, seeds = [b"zeta"], bump)]
    pub zeta_account: Account<'info, ZetaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct ZetaAccount {
    pub m: u8,
    pub n: u32,
    pub o: u64,
    pub bump: u8,
}
