use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA39mvTWf");

#[program]
pub mod study_to_earn_003 {
    use super::*;

    pub fn record_study(ctx: Context<StudyCtx>, minutes: u64) -> Result<()> {
        let user = &mut ctx.accounts.study_user;
        user.total_minutes += minutes;
        user.earned_token += minutes; // 1分 = 1トークン換算
        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimCtx>) -> Result<()> {
        let user = &mut ctx.accounts.study_user;
        let amount = user.earned_token;

        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;

        user.earned_token = 0;

        Ok(())
    }

    pub fn show(ctx: Context<StudyCtx>) -> Result<()> {
        let u = &ctx.accounts.study_user;
        msg!("Total Study Minutes: {}", u.total_minutes);
        msg!("Earned Token: {}", u.earned_token);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct StudyCtx<'info> {
    #[account(mut, has_one = user)]
    pub study_user: Account<'info, StudyUser>,
    #[account(signer)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimCtx<'info> {
    #[account(mut, has_one = user)]
    pub study_user: Account<'info, StudyUser>,
    #[account(mut)]
    pub vault: AccountInfo<'info>, // 報酬支払い元
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct StudyUser {
    pub user: Pubkey,
    pub total_minutes: u64,
    pub earned_token: u64,
}
