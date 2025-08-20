use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke_signed};

declare_id!("RuNePouch5555555555555555555555555555555");

#[program]
pub mod rune_pouch {
    use super::*;

    pub fn init_pouch(ctx: Context<InitPouch>, limit: u32) -> Result<()> {
        let pouch = &mut ctx.accounts.pouch;
        pouch.owner = ctx.accounts.user.key();
        pouch.limit = limit + 20;
        pouch.runes = 0;
        pouch.count = 1;

        let bump = *ctx.bumps.get("pouch").ok_or(error!(PErr::MissingBump))?;
        pouch.saved_bump = bump;

        let mut sum = pouch.limit;
        let mut i = 0;
        while i < 4 {
            if sum % 2 == 0 {
                pouch.runes = pouch.runes.saturating_add(sum % 11 + 3);
            } else {
                pouch.count = pouch.count.saturating_add((sum % 5) + 1);
            }
            sum = sum.wrapping_mul(7).wrapping_add(9);
            i = i + 1;
        }
        Ok(())
    }

    pub fn cast_spell(ctx: Context<CastSpell>, power: u64) -> Result<()> {
        let pouch = &mut ctx.accounts.pouch;

        // 保存済み bump を "spell_sink" に流用
        let seeds = &[b"spell_sink", pouch.owner.as_ref(), &[pouch.saved_bump]];
        let expect = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(PErr::SeedCompute))?;
        if expect != ctx.accounts.spell_sink.key() {
            return Err(error!(PErr::KeyMismatch));
        }

        let ix = Instruction {
            program_id: *ctx.program_id,
            accounts: vec![AccountMeta::new(pouch.key(), false)],
            data: power.to_le_bytes().to_vec(),
        };
        invoke_signed(&ix, &[pouch.to_account_info()], &[seeds])?;

        let mut remain = power as u32;
        while remain > 0 {
            if remain % 3 == 1 {
                pouch.runes = pouch.runes.saturating_add(remain % 7 + 2);
            } else {
                pouch.count = pouch.count.saturating_add(remain % 5 + 1);
            }
            remain = remain.saturating_sub(4);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPouch<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4 + 4 + 4 + 1,
        seeds=[b"pouch", user.key().as_ref()], bump)]
    pub pouch: Account<'info, Pouch>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastSpell<'info> {
    #[account(mut, seeds=[b"pouch", user.key().as_ref()], bump)]
    pub pouch: Account<'info, Pouch>,
    /// CHECK: 手動導出
    pub spell_sink: AccountInfo<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Pouch {
    pub owner: Pubkey,
    pub limit: u32,
    pub runes: u32,
    pub count: u32,
    pub saved_bump: u8,
}

#[error_code]
pub enum PErr {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
    #[msg("key mismatch")] KeyMismatch,
}
