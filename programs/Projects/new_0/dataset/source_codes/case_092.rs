use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfCFDGHT");

#[program]
pub mod crowdfunding {
    use super::*;

    /// 新しいキャンペーンを初期化（管理者が署名）
    pub fn initialize_campaign(
        ctx: Context<InitializeCampaign>,
        goal: u64,
    ) -> Result<()> {
        let camp = &mut ctx.accounts.campaign;
        camp.organizer = ctx.accounts.organizer.key();
        camp.goal = goal;
        camp.raised = 0;
        msg!("Campaign initialized by {}, goal = {}", camp.organizer, camp.goal);
        Ok(())
    }

    /// 支援 (誰でも可／署名必須)
    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        require!(
            ctx.accounts.contributor.is_signer,
            ErrorCode::Unauthorized
        );
        // lamports移動
        **ctx.accounts.contributor.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()?         += amount;
        // 状態更新
        let camp = &mut ctx.accounts.campaign;
        camp.raised = camp.raised.checked_add(amount).unwrap();
        msg!(
            "{} contributed {} lamports (total raised = {})",
            ctx.accounts.contributor.key(),
            amount,
            camp.raised
        );
        Ok(())
    }

    /// 目標達成後に資金を引き出し（主催者のみ）
    pub fn withdraw_funds(ctx: Context<WithdrawFunds>) -> Result<()> {
        require!(
            ctx.accounts.organizer.is_signer,
            ErrorCode::Unauthorized
        );
        let camp_balance = **ctx.accounts.vault.to_account_info().lamports.borrow();
        require!(camp_balance >= ctx.accounts.campaign.goal, ErrorCode::GoalNotReached);
        // 全額送金
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()?      -= camp_balance;
        **ctx.accounts.organizer.to_account_info().try_borrow_mut_lamports()?  += camp_balance;
        msg!(
            "Organizer {} withdrew {} lamports",
            ctx.accounts.organizer.key(),
            camp_balance
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCampaign<'info> {
    #[account(
        init,
        payer = organizer,
        space = 8 + 32 + 8 + 8,
        seeds = [b"camp", organizer.key().as_ref()],
        bump
    )]
    pub campaign:  Account<'info, Campaign>,
    #[account(mut)]
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(
        mut,
        seeds = [b"camp", campaign.organizer.as_ref()],
        bump,
    )]
    pub campaign:   Account<'info, Campaign>,
    #[account(mut)]
    pub contributor: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", campaign.key().as_ref()],
        bump,
        init_if_needed,
        payer = contributor,
        space = 8
    )]
    pub vault:      SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(
        mut,
        seeds = [b"camp", organizer.key().as_ref()],
        bump,
        has_one = organizer
    )]
    pub campaign:   Account<'info, Campaign>,
    pub organizer:  Signer<'info>,
    #[account(mut,
        seeds = [b"vault", campaign.key().as_ref()],
        bump
    )]
    pub vault:      SystemAccount<'info>,
}

#[account]
pub struct Campaign {
    pub organizer: Pubkey,
    pub goal:      u64,
    pub raised:    u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: signer required")]
    Unauthorized,
    #[msg("Goal not yet reached")]
    GoalNotReached,
}
