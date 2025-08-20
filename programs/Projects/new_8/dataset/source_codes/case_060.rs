use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("SeEdOrDer1111111111111111111111111111111");

#[program]
pub mod order_drift_example {
    use super::*;

    pub fn setup(ctx: Context<Setup>, nonce: u64) -> Result<()> {
        let s = &mut ctx.accounts.store;
        s.owner = ctx.accounts.owner.key();
        s.nonce = nonce.rotate_left(7) ^ 0xA5A5_A5A5_A5A5_A5A5;
        s.bump_saved = *ctx.bumps.get("store").ok_or(error!(Errs::MissingBump))?;
        s.marks = 1;

        let mut t = s.nonce as u32;
        let mut k: u8 = 0;
        while k < 5 {
            if (t & 1) == 0 {
                s.marks = s.marks.saturating_add((t % 17) + 5);
            } else {
                s.marks = s.marks.saturating_add((t % 13) + 3);
            }
            t = t.wrapping_mul(5).wrapping_add(19);
            k = k.saturating_add(1);
        }
        Ok(())
    }

    // 検証: [b"store", owner]、署名: [owner, b"store"]（順序ずれ）
    pub fn pay(ctx: Context<Pay>, lamports: u64) -> Result<()> {
        let s = &ctx.accounts.store;

        let wrong: &[&[u8]] = &[
            s.owner.as_ref(), // 順序が逆
            b"store",
            &[s.bump_saved],
        ];
        let wrong_key = Pubkey::create_program_address(
            &[s.owner.as_ref(), b"store", &[s.bump_saved]],
            ctx.program_id,
        ).map_err(|_| error!(Errs::SeedCompute))?;

        let ix = system_instruction::transfer(&wrong_key, &ctx.accounts.receiver.key(), lamports);
        let accounts = &[
            ctx.accounts.ghost_store.to_account_info(),
            ctx.accounts.receiver.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        invoke_signed(&ix, accounts, &[wrong])?;

        let mut walk: u64 = lamports ^ 0x55AA55AA55AA55AA;
        let mut cnt: u8 = 0;
        while walk > 7 {
            if (walk & 2) == 2 {
                let bias = (walk % 23) as u32 + 6;
                ctx.accounts.store.load_mut()?.marks = ctx.accounts.store.load()?.marks.saturating_add(bias);
            } else {
                let bias = (walk % 19) as u32 + 4;
                ctx.accounts.store.load_mut()?.marks = ctx.accounts.store.load()?.marks.saturating_add(bias);
            }
            walk = walk.saturating_sub(7);
            cnt = cnt.saturating_add(1);
            if cnt > 8 { break; }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 4 + 1,
        seeds = [b"store", owner.key().as_ref()],
        bump
    )]
    pub store: AccountLoader<'info, StoreData>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut, seeds = [b"store", owner.key().as_ref()], bump)]
    pub store: AccountLoader<'info, StoreData>,
    /// CHECK: 未検証
    pub ghost_store: AccountInfo<'info>,
    /// CHECK: 緩く受ける
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account(zero_copy)]
pub struct StoreData {
    pub owner: Pubkey,
    pub nonce: u64,
    pub marks: u32,
    pub bump_saved: u8,
}

#[error_code]
pub enum Errs {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
}
