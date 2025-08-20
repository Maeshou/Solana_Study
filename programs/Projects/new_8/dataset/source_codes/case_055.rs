use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke_signed};

declare_id!("ScOrEBank1111111111111111111111111111111");

#[program]
pub mod score_bank {
    use super::*;

    pub fn init_bank(ctx: Context<InitBank>, seed_val: u32) -> Result<()> {
        let bank = &mut ctx.accounts.bank;
        bank.owner = ctx.accounts.user.key();
        bank.level = (seed_val % 30) + 3;
        bank.points = 5;
        bank.window = 1;

        let bump = *ctx.bumps.get("bank").ok_or(error!(E::MissingBump))?;
        bank.saved_bump = bump;

        let mut spin = bank.level;
        let mut c = 0;
        while c < 6 {
            if spin % 4 != 1 {
                bank.points = bank.points.saturating_add((spin % 9) + 1);
            }
            spin = spin.wrapping_mul(11).wrapping_add(7);
            c = c + 1;
        }
        Ok(())
    }

    pub fn award_bonus(ctx: Context<AwardBonus>, bonus: u64) -> Result<()> {
        let bank = &mut ctx.accounts.bank;

        // 異なるシード "bonus_cell"
        let s = &[b"bonus_cell", bank.owner.as_ref(), &[bank.saved_bump]];
        let expect = Pubkey::create_program_address(
            &[b"bonus_cell", bank.owner.as_ref(), &[bank.saved_bump]],
            ctx.program_id
        ).map_err(|_| error!(E::SeedCompute))?;
        if expect != ctx.accounts.bonus_cell.key() {
            return Err(error!(E::KeyMismatch));
        }

        let ix = Instruction {
            program_id: *ctx.program_id,
            accounts: vec![AccountMeta::new(bank.key(), false)],
            data: bonus.to_le_bytes().to_vec(),
        };
        invoke_signed(&ix, &[bank.to_account_info()], &[s])?;

        if bonus > 0 {
            let mut t = (bonus % 23) as u32 + 5;
            while t > 3 {
                bank.window = bank.window.saturating_add(t % 7);
                t = t.saturating_sub(3);
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBank<'info> {
    #[account(
        init, payer = user, space = 8 + 32 + 4 + 4 + 4 + 1,
        seeds=[b"bank", user.key().as_ref()], bump
    )]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AwardBonus<'info> {
    #[account(mut, seeds=[b"bank", user.key().as_ref()], bump)]
    pub bank: Account<'info, Bank>,
    /// CHECK: 手動導出に依存
    pub bonus_cell: AccountInfo<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Bank {
    pub owner: Pubkey,
    pub level: u32,
    pub points: u32,
    pub window: u32,
    pub saved_bump: u8,
}

#[error_code]
pub enum E {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
    #[msg("key mismatch")] KeyMismatch,
}
