use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfSUBSCR");

#[program]
pub mod subscription_service {
    use super::*;

    /// サービス管理者だけが呼べる：サブスクプラン設定
    pub fn initialize_plan(
        ctx: Context<InitializePlan>,
        price_per_period: u64,
    ) -> Result<()> {
        let plan = &mut ctx.accounts.plan;
        plan.admin = ctx.accounts.admin.key();
        plan.price = price_per_period;
        Ok(())
    }

    /// ユーザーは署名付きで契約を開始
    pub fn subscribe(
        ctx: Context<Subscribe>,
        periods: u8,
    ) -> Result<()> {
        // 必要な支払いをチェック
        let cost = ctx.accounts.plan.price.checked_mul(periods as u64).unwrap();
        require!(
            ctx.accounts.payer.to_account_info().lamports() >= cost,
            ErrorCode::InsufficientFunds
        );
        **ctx.accounts.payer.to_account_info().try_borrow_mut_lamports()? -= cost;
        **ctx.accounts.plan.to_account_info().try_borrow_mut_lamports()? += cost;

        let sub = &mut ctx.accounts.subscription;
        sub.user = ctx.accounts.payer.key();
        sub.periods = periods;
        Ok(())
    }

    /// ユーザーのサブスク情報を表示
    pub fn view_subscription(
        ctx: Context<ViewSubscription>,
    ) -> Result<()> {
        let sub = &ctx.accounts.subscription;
        msg!(
            "User {} has {} period(s) remaining",
            sub.user,
            sub.periods
        );
        Ok(())
    }

    /// 管理者が価格を更新
    pub fn update_price(
        ctx: Context<UpdatePrice>,
        new_price: u64,
    ) -> Result<()> {
        // 管理者署名チェック
        require!(
            ctx.accounts.admin.is_signer,
            ErrorCode::Unauthorized
        );
        ctx.accounts.plan.price = new_price;
        msg!("Plan price updated to {}", new_price);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePlan<'info> {
    /// PDA でプランアカウントを初期化
    #[account(
        init,
        payer = admin,
        space  = 8 + 32 + 8,
        seeds  = [b"plan", admin.key().as_ref()],
        bump
    )]
    pub plan: Account<'info, PlanAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(mut)]
    pub plan: Account<'info, PlanAccount>,

    /// ユーザー署名付き
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space  = 8 + 32 + 1,
        seeds  = [b"sub", plan.key().as_ref(), payer.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, SubscriptionAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ViewSubscription<'info> {
    #[account(
        seeds = [b"sub", plan.key().as_ref(), user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub subscription: Account<'info, SubscriptionAccount>,

    pub user: Signer<'info>,
    pub plan: Account<'info, PlanAccount>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(
        mut,
        seeds = [b"plan", admin.key().as_ref()],
        bump,
        has_one = admin
    )]
    pub plan: Account<'info, PlanAccount>,

    pub admin: Signer<'info>,
}

#[account]
pub struct PlanAccount {
    pub admin: Pubkey,
    pub price: u64,
}

#[account]
pub struct SubscriptionAccount {
    pub user: Pubkey,
    pub periods: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds to subscribe")]
    InsufficientFunds,
    #[msg("Unauthorized: signer required")]
    Unauthorized,
}
