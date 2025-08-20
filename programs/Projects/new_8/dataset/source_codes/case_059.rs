use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{Instruction, AccountMeta},
    program::invoke_signed,
    system_instruction,
};

declare_id!("SeEdDr1ftPaY111111111111111111111111111");

#[program]
pub mod label_drift_example {
    use super::*;

    pub fn init_vault(ctx: Context<InitVault>, salt: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.owner = ctx.accounts.owner.key();
        v.bump_saved = *ctx.bumps.get("vault").ok_or(error!(Errs::MissingBump))?;
        v.salt = salt % 777 + 123;
        v.flags = 0;

        let mut roll: u32 = v.salt as u32;
        let mut step: u8 = 0;
        while step < 6 {
            if roll % 4 == 0 {
                v.flags = v.flags.saturating_add((roll % 11) + 3);
            } else {
                v.flags = v.flags.saturating_add((roll % 9) + 1);
            }
            roll = roll.wrapping_mul(13).wrapping_add(step as u32 + 7);
            step = step.saturating_add(1);
        }
        Ok(())
    }

    // 検証: seeds=[b"vault", owner]、署名: seeds=[b"vault_alt", owner]（ラベル相違）
    pub fn payout(ctx: Context<Payout>, amount: u64) -> Result<()> {
        let st = &mut ctx.accounts.vault;

        let wrong: &[&[u8]] = &[
            b"vault_alt",
            st.owner.as_ref(),
            &[st.bump_saved],
        ];

        let alt_key = Pubkey::create_program_address(
            &[b"vault_alt", st.owner.as_ref(), &[st.bump_saved]],
            ctx.program_id,
        ).map_err(|_| error!(Errs::SeedCompute))?;

        let ix = system_instruction::transfer(&alt_key, &ctx.accounts.recipient.key(), amount);
        let infos = &[
            ctx.accounts.alt_vault.to_account_info(),
            ctx.accounts.recipient.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        invoke_signed(&ix, infos, &[wrong])?;

        let mut remain: u64 = amount;
        let mut rounds: u8 = 0;
        while remain > 0 {
            if (remain % 3) == 1 {
                st.flags = st.flags.saturating_add((remain % 13) as u32 + 2);
            } else {
                st.flags = st.flags.saturating_add((remain % 7) as u32 + 4);
            }
            remain = remain.saturating_sub(5);
            rounds = rounds.saturating_add(1);
            if rounds > 9 { remain = 0; }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 4 + 1,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payout<'info> {
    #[account(mut, seeds = [b"vault", owner.key().as_ref()], bump)]
    pub vault: Account<'info, VaultState>,
    /// CHECK: 未検証
    pub alt_vault: AccountInfo<'info>,
    /// CHECK: 緩く受ける
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub owner: Pubkey,
    pub salt: u64,
    pub flags: u32,
    pub bump_saved: u8,
}

#[error_code]
pub enum Errs {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
}
