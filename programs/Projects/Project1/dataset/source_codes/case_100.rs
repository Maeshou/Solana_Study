use anchor_lang::prelude::*;

/// 募金プログラム
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfDONATE");

#[program]
pub mod donation_program {
    use super::*;

    /// 募金用アカウントを初期化します（管理者のみ実行可）
    pub fn init_fund(ctx: Context<InitFund>) -> Result<()> {
        let fund = &mut ctx.accounts.fund;
        fund.admin           = ctx.accounts.admin.key();
        fund.total_donations = 0;
        msg!("Fund initialized by {}", fund.admin);
        Ok(())
    }

    /// 任意のユーザーが募金（Lamports転送）できる
    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        // Signer 制約で donor の署名を担保済
        **ctx.accounts.donor.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.fund.to_account_info().try_borrow_mut_lamports()?  += amount;
        let fund = &mut ctx.accounts.fund;
        fund.total_donations = fund.total_donations.checked_add(amount).unwrap();
        msg!("{} donated {} lamports (total: {})",
             ctx.accounts.donor.key(),
             amount,
             fund.total_donations);
        Ok(())
    }
}

/// 募金プログラム用アカウント構造体
#[account]
pub struct Fund {
    pub admin:           Pubkey,
    pub total_donations: u64,
    pub bump:            u8,
}

/// 初期化時のAccounts
#[derive(Accounts)]
pub struct InitFund<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 8 + 1,
        seeds = [b"fund", admin.key().as_ref()],
        bump
    )]
    pub fund:          Account<'info, Fund>,
    #[account(mut, signer)]
    pub admin:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// 募金時のAccounts
#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(
        mut,
        seeds = [b"fund", fund.admin.as_ref()],
        bump = fund.bump,
        has_one = admin @ ErrorCode::Unauthorized
    )]
    pub fund:  Account<'info, Fund>,
    /// 誰でも募金できるので署名のみチェック
    #[account(mut, signer)]
    pub donor: AccountInfo<'info>,
    /// Fund の所有者（管理者）は募金から除外していないが、参照用
    pub admin: SystemAccount<'info>,
}

/// エラー定義
#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
}
