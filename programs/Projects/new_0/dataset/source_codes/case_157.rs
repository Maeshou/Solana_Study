use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUD");

#[program]
pub mod counter_app {
    use super::*;

    /// カウンター初期化：owner・bump・count をまとめて設定
    pub fn initialize(
        ctx: Context<InitializeCounter>,
        bump: u8,
    ) -> Result<()> {
        // まとめて struct リテラルでフィールドを初期化
        *ctx.accounts.counter = Counter {
            owner: ctx.accounts.user.key(),
            bump,
            count: 0,
        };
        Ok(())
    }

    /// カウント増加：wrapping_add でオーバーフローも安全に
    pub fn increment(ctx: Context<ModifyCounter>) -> Result<()> {
        let c = &mut ctx.accounts.counter;
        c.count = c.count.wrapping_add(1);
        Ok(())
    }

    /// カウントリセット
    pub fn reset(ctx: Context<ModifyCounter>) -> Result<()> {
        ctx.accounts.counter.count = 0;
        Ok(())
    }

    /// アカウント閉鎖
    pub fn close(ctx: Context<CloseCounter>) -> Result<()> {
        // close 属性により自動でアカウント解放＆残高返却
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeCounter<'info> {
    /// PDA で生成する Counter アカウント
    #[account(
        init,
        payer = user,
        // 8 + 32 + 1 + 8 = discriminator + owner + bump + count
        space = 8 + 32 + 1 + 8,
        seeds = [b"counter", user.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, Counter>,

    /// カウンター所有者（署名必須）
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyCounter<'info> {
    /// 既存の Counter（PDA／bump／オーナーチェック）
    #[account(
        mut,
        seeds = [b"counter", owner.key().as_ref()],
        bump = counter.bump,
        has_one = owner
    )]
    pub counter: Account<'info, Counter>,

    /// カウンター所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseCounter<'info> {
    /// 閉鎖対象の Counter（PDA／bump／has_one／close）
    #[account(
        mut,
        seeds = [b"counter", owner.key().as_ref()],
        bump = counter.bump,
        has_one = owner,
        close = owner
    )]
    pub counter: Account<'info, Counter>,

    /// カウンター所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub owner: Pubkey,
    pub bump: u8,
    pub count: u64,
}
