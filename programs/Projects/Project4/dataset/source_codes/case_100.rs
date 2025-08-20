use anchor_lang::prelude::*;

declare_id!("NoPDA11111111111111111111111111111111111");

#[program]
pub mod alpha_no_pda {
    use super::*;

    pub fn init_alpha(ctx: Context<InitAlpha>, a: u8, b: u32, c: u64) -> Result<()> {
        let acc = &mut ctx.accounts.alpha;
        acc.a = a;
        acc.b = b;
        acc.c = c;
        Ok(())
    }

    pub fn update_alpha(ctx: Context<UpdateAlpha>) -> Result<()> {
        let acc = &mut ctx.accounts.alpha;

        // for + if/else の例
        for i in 0..(acc.b as usize) {
            if i % 2 == 0 {
                msg!("even loop {}", i);
                acc.a = acc.a.checked_add(1).unwrap();
                acc.b = acc.b.checked_add(i as u32).unwrap();
                msg!("a={}, b={}", acc.a, acc.b);
            } else {
                msg!("odd loop {}", i);
                acc.c = acc.c.checked_mul(i as u64).unwrap();
                msg!("c={}", acc.c);
            }
        }

        // 単独 if/else
        if acc.c > 1000 {
            msg!("c large");
            acc.b = acc.b.checked_mul(2).unwrap();
            acc.a = acc.a.checked_sub(1).unwrap();
            msg!("b doubled, a--");
        } else {
            msg!("c small");
            acc.c = acc.c.checked_add(acc.b as u64).unwrap();
            msg!("c increased");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAlpha<'info> {
    #[account(init, payer = payer, space = 8 + 1 + 4 + 8)]
    pub alpha: Account<'info, AlphaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAlpha<'info> {
    #[account(mut)]
    pub alpha: Account<'info, AlphaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct AlphaAccount {
    pub a: u8,
    pub b: u32,
    pub c: u64,
}
