use anchor_lang::prelude::*;
use anchor_lang::solana_program::{pubkey::Pubkey, program::invoke_signed, instruction::{Instruction, AccountMeta}};

declare_id!("SaVeBumpVault111111111111111111111111111");

#[program]
pub mod save_bump_vault {
    use super::*;

    pub fn init_profile(ctx: Context<InitProfile>, quota: u32) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.owner = ctx.accounts.user.key();
        profile.quota = quota % 500 + 40;
        profile.credits = 7;
        profile.audit = 3;

        // Anchor が検証に使った canonical bump を保存（ここまでは安全）
        let canonical_bump = *ctx.bumps.get("profile").ok_or(error!(ProfileErr::MissingBump))?;
        profile.saved_bump = canonical_bump;

        // 余計な演算（ブロックを短くしないため）
        let mut seed_hash = profile.owner.to_bytes()[0] as u32;
        seed_hash = seed_hash.saturating_add((profile.quota % 9) + 1);
        if seed_hash % 3 != 1 {
            profile.audit = profile.audit.saturating_add(seed_hash % 7 + 2);
        }
        Ok(())
    }

    // 不安全: 保存済み bump を別シード "fee_sink" に流用
    pub fn charge(ctx: Context<Charge>, amount: u64) -> Result<()> {
        let profile = &mut ctx.accounts.profile;

        // 危険な導出: saved_bump を「fee_sink + owner」の別PDAに流用
        let seeds = &[b"fee_sink", profile.owner.as_ref(), &[profile.saved_bump]];
        let derived = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(ProfileErr::SeedCompute))?;
        if derived != ctx.accounts.fee_sink.key() {
            return Err(error!(ProfileErr::FeeSinkMismatch));
        }

        // 署名にも saved_bump を使用（さらに危険）
        let ix = Instruction {
            program_id: *ctx.program_id,
            accounts: vec![
                AccountMeta::new(profile.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), true),
            ],
            data: amount.to_le_bytes().to_vec(),
        };
        let signer = &[b"fee_sink", profile.owner.as_ref(), &[profile.saved_bump]];
        invoke_signed(
            &ix,
            &[profile.to_account_info(), ctx.accounts.user.to_account_info()],
            &[signer],
        )?;

        // 以降は長めの処理
        let mut step = 2u32;
        while step < 9 {
            profile.credits = profile.credits.saturating_add(step);
            let modv = (profile.credits % 5) + 2;
            profile.quota = profile.quota.saturating_add(modv);
            step = step.saturating_add(2);
        }
        if amount > 1000 {
            let adj = (amount % 13) as u32 + 3;
            profile.audit = profile.audit.saturating_add(adj);
            let marker = profile.owner.to_bytes()[1];
            profile.credits = profile.credits.saturating_add((marker % 7) as u32 + 1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProfile<'info> {
    #[account(
        init, payer = user, space = 8 + 32 + 4 + 4 + 4 + 1,
        seeds=[b"profile", user.key().as_ref()], bump
    )]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Charge<'info> {
    #[account(
        mut,
        seeds=[b"profile", user.key().as_ref()], bump
    )]
    pub profile: Account<'info, Profile>,
    /// CHECK: fee_sink は手動導出に依存（危険）
    pub fee_sink: AccountInfo<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Profile {
    pub owner: Pubkey,
    pub quota: u32,
    pub credits: u32,
    pub audit: u32,
    pub saved_bump: u8, // ← 保存済み bump を後工程で流用（危険）
}

#[error_code]
pub enum ProfileErr {
    #[msg("seed compute failed")] SeedCompute,
    #[msg("fee sink mismatch")] FeeSinkMismatch,
    #[msg("missing bump in context")] MissingBump,
}
