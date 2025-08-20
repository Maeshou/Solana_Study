use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfDONATE");

#[program]
pub mod simple_donation {
    use super::*;

    /// 初回のみドネーションアカウントを初期化し、累計額を 0 にします。
    pub fn initialize_donation(ctx: Context<InitializeDonation>) -> Result<()> {
        let rec = &mut ctx.accounts.donation_record;
        rec.user = ctx.accounts.user.key();
        rec.total_donated = 0;
        Ok(())
    }

    /// 署名者チェック済みユーザーが任意の額を寄付します（累計に加算）。
    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        let rec = &mut ctx.accounts.donation_record;
        // 署名者チェック済みなので安全に加算
        rec.total_donated = rec.total_donated.saturating_add(amount);
        msg!("User {:?} donated {}, total now {}",
            rec.user, amount, rec.total_donated);
        Ok(())
    }

    /// 累計寄付額をログ出力します。
    pub fn view_total(ctx: Context<ViewTotal>) -> Result<()> {
        let rec = &ctx.accounts.donation_record;
        msg!("User {:?} total donated: {}", rec.user, rec.total_donated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDonation<'info> {
    /// 初回のみ PDA を生成・初期化
    #[account(
        init,
        payer = user,
        space  = 8 + 32 + 8,
        seeds = [b"donation", user.key().as_ref()],
        bump
    )]
    pub donation_record: Account<'info, DonationRecord>,

    /// 操作にはユーザーの署名が必要
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    /// PDA と所有者チェックで不正アクセス防止
    #[account(
        seeds = [b"donation", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub donation_record: Account<'info, DonationRecord>,

    /// 操作にはユーザーの署名が必要
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewTotal<'info> {
    /// PDA と所有者チェックで不正アクセス防止
    #[account(
        seeds = [b"donation", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub donation_record: Account<'info, DonationRecord>,

    /// 操作にはユーザーの署名が必要
    pub user: Signer<'info>,
}

#[account]
pub struct DonationRecord {
    /// レコード所有者
    pub user: Pubkey,
    /// 累計寄付額
    pub total_donated: u64,
}
