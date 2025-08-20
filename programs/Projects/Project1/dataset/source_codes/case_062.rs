use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfUnlock01");

#[program]
pub mod unlock_action_system {
    use super::*;

    // 初期化：所持ポイントを設定
    pub fn initialize_points(ctx: Context<InitializePoints>, initial_points: u64) -> Result<()> {
        let acc = &mut ctx.accounts.profile;
        acc.owner = ctx.accounts.user.key();
        acc.points = initial_points;
        acc.unlocked = false;
        Ok(())
    }

    // アクションを一度だけ解放（10ポイント消費）
    pub fn unlock_action(ctx: Context<UnlockAction>) -> Result<()> {
        let acc = &mut ctx.accounts.profile;

        // 未解放かつ 10ポイント以上あるか検証（分岐なし）
        let unused = (acc.unlocked == false) as u64;
        let enough = (acc.points >= 10) as u64;
        let _ = 1 / (unused * enough); // どちらか false なら panic

        acc.points -= 10;
        acc.unlocked = true;
        Ok(())
    }

    pub fn view_profile(ctx: Context<UnlockAction>) -> Result<()> {
        let acc = &ctx.accounts.profile;
        msg!("Owner: {}", acc.owner);
        msg!("Remaining Points: {}", acc.points);
        msg!("Unlocked: {}", acc.unlocked);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePoints<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 1,
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, PlayerProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnlockAction<'info> {
    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub profile: Account<'info, PlayerProfile>,
    pub user: Signer<'info>,
}

#[account]
pub struct PlayerProfile {
    pub owner: Pubkey,
    pub points: u64,
    pub unlocked: bool,
}
