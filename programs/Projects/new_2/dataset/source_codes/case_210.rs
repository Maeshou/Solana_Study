use anchor_lang::prelude::*;

declare_id!("VulnVarX1000000000000000000000000000000001");

#[program]
pub mod example1 {
    pub fn toggle_feature(ctx: Context<Ctx1>) -> Result<()> {
        // diagnostic_buf は unchecked
        let mut buf = ctx.accounts.diagnostic_buf.data.borrow_mut();
        buf[0] ^= 1; // トグル動作だけログに残す

        // feature_flag は has_one 検証済み
        let flag = &mut ctx.accounts.feature_flag;
        flag.enabled = !flag.enabled;
        flag.toggle_count = flag.toggle_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx1<'info> {
    /// CHECK: デバッグ用バッファ、所有者検証なし
    #[account(mut)]
    pub diagnostic_buf: AccountInfo<'info>,

    #[account(mut, has_one = admin)]
    pub feature_flag: Account<'info, FeatureFlag>,
    pub admin: Signer<'info>,
}

#[account]
pub struct FeatureFlag {
    pub admin: Pubkey,
    pub enabled: bool,
    pub toggle_count: u64,
}
