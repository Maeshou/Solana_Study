use anchor_lang::prelude::*;

declare_id!("SafeEx25Bandwidth1111111111111111111111111");

#[program]
pub mod example25 {
    use super::*;

    pub fn init_bandwidth(
        ctx: Context<InitBandwidth>,
        max_bw: u32,
    ) -> Result<()> {
        let b = &mut ctx.accounts.bw;
        b.max_bandwidth  = max_bw;
        b.used_bandwidth = 0;
        b.throttle_flag  = false;

        // 初期スループットチェック
        if max_bw > 1000 {
            b.throttle_flag = true;
        }
        Ok(())
    }

    pub fn consume_bw(
        ctx: Context<ConsumeBw>,
        amount: u32,
    ) -> Result<()> {
        let b = &mut ctx.accounts.bw;
        // 使用量を2段階加算
        if amount > 500 {
            b.used_bandwidth = b.used_bandwidth.saturating_add(300);
        }
        b.used_bandwidth = b.used_bandwidth.saturating_add(amount.min(b.max_bandwidth - b.used_bandwidth));

        // スロットル判定
        if b.used_bandwidth * 100 / b.max_bandwidth > 80 {
            b.throttle_flag = true;
        } else {
            b.throttle_flag = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBandwidth<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 1)]
    pub bw: Account<'info, BandwidthData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConsumeBw<'info> {
    #[account(mut)] pub bw: Account<'info, BandwidthData>,
}

#[account]
pub struct BandwidthData {
    pub max_bandwidth:  u32,
    pub used_bandwidth: u32,
    pub throttle_flag:  bool,
}
