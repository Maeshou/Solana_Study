use anchor_lang::prelude::*;

declare_id!("Eta7777777777777777777777777777777777777");

#[program]
pub mod eta_example {
    use super::*;

    pub fn init_eta(ctx: Context<InitEta>, u: u8, v: u32, w: u64) -> Result<()> {
        let acc = &mut ctx.accounts.eta_account;
        acc.u = u;
        acc.v = v;
        acc.w = w;
        acc.bump = *ctx.bumps.get("eta_account").unwrap();
        Ok(())
    }

    pub fn update_eta(ctx: Context<UpdateEta>) -> Result<()> {
        let acc = &mut ctx.accounts.eta_account;

        // ループ + 分岐①
        let mut i = 1;
        while i <= (acc.v as usize) {
            if i % 2 == 0 {
                msg!("even loop: {}", i);
                acc.v = acc.v.checked_add(4).unwrap();
                acc.u = acc.u.checked_add(1).unwrap();
                msg!("u & v updated");
            } else {
                msg!("odd loop: {}", i);
                acc.w = acc.w.checked_mul(2).unwrap();
                msg!("w doubled");
            }
            i += 1;
        }

        // 分岐②
        if acc.w < 100 {
            msg!("w low");
            acc.u = acc.u.checked_add(2).unwrap();
            acc.v = acc.v.checked_add(10).unwrap();
            msg!("u+2 & v+10");
        } else {
            msg!("w high");
            acc.w = acc.w.checked_sub(50).unwrap();
            msg!("w-50");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEta<'info> {
    #[account(init, seeds = [b"eta"], bump, payer = payer, space = 8 + 1 + 4 + 8 + 1)]
    pub eta_account: Account<'info, EtaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateEta<'info> {
    #[account(mut, seeds = [b"eta"], bump)]
    pub eta_account: Account<'info, EtaAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct EtaAccount {
    pub u: u8,
    pub v: u32,
    pub w: u64,
    pub bump: u8,
}
