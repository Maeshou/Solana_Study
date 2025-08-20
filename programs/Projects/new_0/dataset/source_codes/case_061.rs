use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfRefr001");

#[program]
pub mod referral_registry {
    use super::*;

    // 初回のみ実行：招待アカウント初期化
    pub fn initialize(ctx: Context<InitializeReferral>) -> Result<()> {
        let acc = &mut ctx.accounts.referral;
        acc.owner = ctx.accounts.user.key();
        acc.referrer = Pubkey::default(); // 未使用状態
        Ok(())
    }

    // 一度だけ招待コード（referrerのPubkey）を登録
    pub fn register_referrer(ctx: Context<RegisterReferral>, referrer: Pubkey) -> Result<()> {
        let acc = &mut ctx.accounts.referral;

        // referrer が未使用状態（デフォルト）でないと panic
        let unused = acc.referrer == Pubkey::default();
        let _ = 1u64 / (unused as u64);

        acc.referrer = referrer;
        Ok(())
    }

    pub fn view(ctx: Context<RegisterReferral>) -> Result<()> {
        let r = &ctx.accounts.referral;
        msg!("Owner: {}", r.owner);
        msg!("Referrer: {}", r.referrer);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeReferral<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32,
        seeds = [b"referral", user.key().as_ref()],
        bump
    )]
    pub referral: Account<'info, ReferralAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterReferral<'info> {
    #[account(
        mut,
        seeds = [b"referral", user.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub referral: Account<'info, ReferralAccount>,
    pub user: Signer<'info>,
}

#[account]
pub struct ReferralAccount {
    pub owner: Pubkey,
    pub referrer: Pubkey,
}
