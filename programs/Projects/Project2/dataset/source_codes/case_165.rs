use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUK");

#[program]
pub mod loyalty_rewards {
    use super::*;

    /// プログラム開始：PDA を生成し、ウェルカムポイント＋初期ランクを設定
    pub fn initialize_rewards(
        ctx: Context<InitializeRewards>,
        bump: u8,
        starting_tier: u8,
    ) -> Result<()> {
        *ctx.accounts.loyalty = Loyalty {
            owner:          ctx.accounts.user.key(),
            bump,
            points_balance: 50,  // ウェルカムボーナス
            tier:            starting_tier,
            action_count:    1,  // 初期化もアクションとしてカウント
            last_action_ts:  ctx.accounts.clock.unix_timestamp,
        };
        Ok(())
    }

    /// ポイント獲得：amount を加算し、アクション数とタイムスタンプを更新
    pub fn earn_points(
        ctx: Context<ModifyRewards>,
        amount: u64,
    ) -> Result<()> {
        let r = &mut ctx.accounts.loyalty;
        r.points_balance = r.points_balance.wrapping_add(amount);
        r.action_count   = r.action_count.wrapping_add(1);
        r.last_action_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// ポイント利用：amount を減算（ゼロ未満はゼロ）、アクション数とタイムスタンプを更新
    pub fn redeem_points(
        ctx: Context<ModifyRewards>,
        amount: u64,
    ) -> Result<()> {
        let r = &mut ctx.accounts.loyalty;
        r.points_balance = r.points_balance.saturating_sub(amount);
        r.action_count   = r.action_count.wrapping_add(1);
        r.last_action_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// ランク変更：新しいランクを設定し、アクション数とタイムスタンプを更新
    pub fn change_tier(
        ctx: Context<ModifyRewards>,
        new_tier: u8,
    ) -> Result<()> {
        let r = &mut ctx.accounts.loyalty;
        r.tier           = new_tier;
        r.action_count   = r.action_count.wrapping_add(1);
        r.last_action_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, starting_tier: u8)]
pub struct InitializeRewards<'info> {
    /// PDA で生成する Loyalty アカウント
    #[account(
        init,
        payer = user,
        // discriminator(8) + owner(32) + bump(1) + points_balance(8)
        // + tier(1) + action_count(4) + last_action_ts(8)
        space = 8 + 32 + 1 + 8 + 1 + 4 + 8,
        seeds = [b"loyalty", user.key().as_ref()],
        bump
    )]
    pub loyalty: Account<'info, Loyalty>,

    /// プログラム利用者（署名必須）
    #[account(mut)]
    pub user: Signer<'info>,

    /// タイムスタンプ取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyRewards<'info> {
    /// 既存の Loyalty（PDA 検証 + オーナーチェック）
    #[account(
        mut,
        seeds = [b"loyalty", owner.key().as_ref()],
        bump = loyalty.bump,
        has_one = owner
    )]
    pub loyalty: Account<'info, Loyalty>,

    /// Loyalty 所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    /// タイムスタンプ取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Loyalty {
    pub owner:          Pubkey,  // アカウント所有者
    pub bump:           u8,      // PDA 用バンプ
    pub points_balance: u64,     // 保持ポイント
    pub tier:           u8,      // 会員ランク
    pub action_count:   u32,     // 操作回数
    pub last_action_ts: i64,     // 最終操作タイムスタンプ
}
