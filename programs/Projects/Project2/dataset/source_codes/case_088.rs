use anchor_lang::prelude::*;

declare_id!("Referral1919191919191919191919191919191919");

#[program]
pub mod referral_handler {
    use super::*;

    /// 初期化：報酬率を設定
    pub fn init_ref(ctx: Context<InitRef>, rate_bps: u16) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        cfg.rate = rate_bps;
        cfg.counts = std::collections::BTreeMap::new();
        emit!(ReferralInitialized { rate_bps });
        Ok(())
    }

    /// リファラル登録＆報酬計算
    pub fn refer(ctx: Context<Refer>, amount: u64) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        let referrer = ctx.accounts.referrer.key();
        let entry = cfg.counts.entry(referrer).or_insert(0);
        *entry = entry.checked_add(amount).unwrap();
        let reward = amount * cfg.rate as u64 / 10_000;
        **ctx.accounts.referrer.to_account_info().lamports.borrow_mut() += reward;
        emit!(Referred { referrer, amount, reward });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRef<'info> {
    #[account(init, payer = payer, space = 8 + 2 + 4 + (32 + 8) * 10)]
    pub config: Account<'info, ReferralConfig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Refer<'info> {
    #[account(mut)]
    pub config: Account<'info, ReferralConfig>,
    pub referrer: Signer<'info>,
}

#[account]
pub struct ReferralConfig {
    pub rate: u16,
    pub counts: std::collections::BTreeMap<Pubkey, u64>,
}

#[event]
pub struct ReferralInitialized {
    pub rate_bps: u16,
}

#[event]
pub struct Referred {
    pub referrer: Pubkey,
    pub amount: u64,
    pub reward: u64,
}
