use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfVSTNTG");

#[program]
pub mod token_vesting {
    use super::*;

    /// ベスティング設定：管理者が受益者と期間・総量を指定
    pub fn initialize_vesting(
        ctx: Context<InitializeVesting>,
        start_ts: i64,
        duration: i64,
        total_amount: u64,
    ) -> Result<()> {
        require!(ctx.accounts.admin.is_signer, ErrorCode::Unauthorized);
        let v = &mut ctx.accounts.vesting;
        v.admin = ctx.accounts.admin.key();
        v.beneficiary = ctx.accounts.beneficiary.key();
        v.start_ts = start_ts;
        v.duration = duration;
        v.total_amount = total_amount;
        v.claimed = 0;
        msg!(
            "Vesting initialized: {} -> {}, start {}, dur {}, amt {}",
            v.admin, v.beneficiary, v.start_ts, v.duration, v.total_amount
        );
        Ok(())
    }

    /// 受益者による請求：経過分を比例配分
    pub fn claim_vested(ctx: Context<ClaimVested>) -> Result<()> {
        require!(ctx.accounts.beneficiary.is_signer, ErrorCode::Unauthorized);
        let clock = Clock::get()?;
        let elapsed = clock.unix_timestamp.checked_sub(ctx.accounts.vesting.start_ts)
            .unwrap()
            .min(ctx.accounts.vesting.duration)
            .max(0);
        // 配分額 = total_amount * elapsed / duration － すでに請求済分
        let vested = (ctx.accounts.vesting.total_amount as u128)
            .checked_mul(elapsed as u128).unwrap()
            .checked_div(ctx.accounts.vesting.duration as u128).unwrap() as u64;
        let amount = vested.checked_sub(ctx.accounts.vesting.claimed).unwrap();
        require!(amount > 0, ErrorCode::NothingToClaim);

        // PDAから受益者へ送金
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()?  -= amount;
        **ctx.accounts.beneficiary.to_account_info().try_borrow_mut_lamports()? += amount;

        // 更新
        ctx.accounts.vesting.claimed = ctx.accounts.vesting.claimed.checked_add(amount).unwrap();
        msg!(
            "{} claimed {} tokens (total claimed {})",
            ctx.accounts.beneficiary.key(),
            amount,
            ctx.accounts.vesting.claimed
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVesting<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32*2 + 8*3 + 8,
        seeds = [b"vest", beneficiary.key().as_ref()],
        bump
    )]
    pub vesting:     Account<'info, Vesting>,
    #[account(mut)]
    pub admin:       Signer<'info>,
    /// トークン受益者 (請求時に署名必須)
    pub beneficiary: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimVested<'info> {
    #[account(
        mut,
        seeds = [b"vest", vesting.beneficiary.as_ref()],
        bump,
        has_one = beneficiary
    )]
    pub vesting:     Account<'info, Vesting>,
    #[account(mut)]
    pub vault:       SystemAccount<'info>,
    pub beneficiary: Signer<'info>,
}

#[account]
pub struct Vesting {
    pub admin:         Pubkey,
    pub beneficiary:   Pubkey,
    pub start_ts:      i64,
    pub duration:      i64,
    pub total_amount:  u64,
    pub claimed:       u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: signer required")]
    Unauthorized,
    #[msg("Nothing to claim at this time")]
    NothingToClaim,
}
