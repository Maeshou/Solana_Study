use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfVSTNG");

#[program]
pub mod vesting_scheduler {
    use super::*;

    /// ベスティングスケジュールを初期化する
    pub fn initialize_vest(
        ctx: Context<InitializeVest>,
        total_amount: u64,
        release_timestamp: i64,
    ) -> Result<()> {
        let vest = &mut ctx.accounts.vest;
        vest.beneficiary      = ctx.accounts.beneficiary.key();
        vest.total_amount     = total_amount;
        vest.released_amount  = 0;
        vest.release_time     = release_timestamp;
        msg!(
            "Vesting for {}: {} tokens at {}",
            vest.beneficiary,
            vest.total_amount,
            vest.release_time
        );
        Ok(())
    }

    /// ベスティングが満了したら受益者が請求する
    pub fn claim_vested(ctx: Context<ClaimVest>) -> Result<()> {
        require!(
            ctx.accounts.beneficiary.is_signer,
            ErrorCode::Unauthorized
        );
        let clock = Clock::get()?.unix_timestamp;
        let vest  = &mut ctx.accounts.vest;
        // シンプル：現在時刻 >= release_time なら全額解放
        let remaining = vest
            .total_amount
            .checked_sub(vest.released_amount)
            .unwrap();
        require!(clock >= vest.release_time, ErrorCode::NotYet);
        vest.released_amount = vest.total_amount;
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= remaining;
        **ctx.accounts.beneficiary.to_account_info().try_borrow_mut_lamports()? += remaining;
        msg!("{} claimed {} tokens", vest.beneficiary, remaining);
        Ok(())
    }

    /// 管理者がベスティングタイムを延長する
    pub fn extend_vesting(ctx: Context<ExtendVest>, extra_secs: i64) -> Result<()> {
        require!(
            ctx.accounts.manager.is_signer,
            ErrorCode::Unauthorized
        );
        let vest = &mut ctx.accounts.vest;
        vest.release_time = vest.release_time.checked_add(extra_secs).unwrap();
        msg!(
            "Vesting for {} extended by {}s to {}",
            vest.beneficiary,
            extra_secs,
            vest.release_time
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVest<'info> {
    #[account(
        init,
        payer = manager,
        space = 8 + 32 + 8 + 8 + 8,
        seeds = [b"vest", beneficiary.key().as_ref()],
        bump
    )]
    pub vest:        Account<'info, Vesting>,
    #[account(mut)]
    pub manager:     Signer<'info>,
    /// ベスティング対象受益者
    pub beneficiary: UncheckedAccount<'info>,
    /// トークンを保管するPDA
    #[account(
        init_if_needed,
        payer = manager,
        space = 8,
        seeds = [b"vault", vest.key().as_ref()],
        bump
    )]
    pub vault:       SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimVest<'info> {
    #[account(
        mut,
        seeds = [b"vest", vest.beneficiary.as_ref()],
        bump,
        has_one = beneficiary
    )]
    pub vest:        Account<'info, Vesting>,
    /// ベスティング受益者自ら請求
    pub beneficiary: Signer<'info>,
    /// トークンを保管するPDA
    #[account(
        mut,
        seeds = [b"vault", vest.key().as_ref()],
        bump
    )]
    pub vault:       SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct ExtendVest<'info> {
    #[account(
        mut,
        seeds = [b"vest", vest.beneficiary.as_ref()],
        bump
    )]
    pub vest:    Account<'info, Vesting>,
    /// 管理者署名
    pub manager: Signer<'info>,
}

#[account]
pub struct Vesting {
    pub beneficiary:     Pubkey,
    pub total_amount:    u64,
    pub released_amount: u64,
    pub release_time:    i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: signer required")]
    Unauthorized,
    #[msg("Not yet vested")]
    NotYet,
}
