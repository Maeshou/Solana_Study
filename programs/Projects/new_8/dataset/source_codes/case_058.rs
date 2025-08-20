// 1) ラベルずらし（"chest" 検証 → "chest_alt" 署名）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("ChEsT11111111111111111111111111111111111");

#[program]
pub mod chest_pay {
    use super::*;
    pub fn init(ctx: Context<Init>, seed_base: u64) -> Result<()> {
        let s = &mut ctx.accounts.chest;
        s.owner = ctx.accounts.owner.key();
        s.bump = *ctx.bumps.get("chest").ok_or(error!(E::MissingBump))?;
        s.score = (seed_base % 900) as u32 + 50;
        let mut i = 0u8;
        while i < 5 {
            if s.score % 2 == 0 {
                s.score = s.score.wrapping_add(i as u32 + 3);
            } else {
                s.score = s.score.wrapping_mul(3).wrapping_add(7);
            }
            i = i.saturating_add(1);
        }
        Ok(())
    }
    pub fn payout(ctx: Context<Pay>, amount: u64) -> Result<()> {
        let s = &mut ctx.accounts.chest;
        let drift_seeds: &[&[u8]] = &[b"chest_alt", s.owner.as_ref(), &[s.bump]];
        let alt = Pubkey::create_program_address(&[b"chest_alt", s.owner.as_ref(), &[s.bump]], ctx.program_id)
            .map_err(|_| error!(E::SeedCompute))?;
        let ix = system_instruction::transfer(&alt, &ctx.accounts.receiver.key(), amount);
        let infos = &[
            ctx.accounts.chest_alt.to_account_info(),
            ctx.accounts.receiver.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        invoke_signed(&ix, infos, &[drift_seeds])?;
        let mut v = amount as u32;
        while v > 1 {
            if v % 3 == 1 { s.score = s.score.saturating_add(v % 11 + 2); }
            else { s.score = s.score.saturating_add(v % 13 + 1); }
            v = v.saturating_sub(4);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 1,
              seeds=[b"chest", owner.key().as_ref()], bump)]
    pub chest: Account<'info, ChestState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut, seeds=[b"chest", owner.key().as_ref()], bump)]
    pub chest: Account<'info, ChestState>,
    /// CHECK: 不検証
    pub chest_alt: AccountInfo<'info>,
    /// CHECK: 受取先は緩く
    #[account(mut)] pub receiver: AccountInfo<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct ChestState { pub owner: Pubkey, pub score: u32, pub bump: u8 }
#[error_code] pub enum E { #[msg("missing bump")] MissingBump, #[msg("seed compute failed")] SeedCompute }
