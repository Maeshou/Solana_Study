use anchor_lang::prelude::*;

declare_id!("OwnChkE4000000000000000000000000000000005");

#[program]
pub mod yield_farm {
    pub fn harvest(
        ctx: Context<Harvest>,
    ) -> Result<()> {
        let farm = &mut ctx.accounts.farm;
        // 属性レベルで farmer を検証
        let reward = farm.calculate_reward();
        farm.claimed = farm.claimed.saturating_add(reward);

        // harvest_buf は unchecked
        ctx.accounts.harvest_buf.data.borrow_mut().extend_from_slice(&reward.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Harvest<'info> {
    #[account(mut, has_one = farmer)]
    pub farm: Account<'info, FarmData>,
    pub farmer: Signer<'info>,
    /// CHECK: 収穫バッファ、所有者検証なし
    #[account(mut)]
    pub harvest_buf: AccountInfo<'info>,
}

#[account]
pub struct FarmData {
    pub farmer: Pubkey,
    pub claimed: u64,
}

impl FarmData {
    pub fn calculate_reward(&self) -> u64 {
        // ダミー計算
        (self.claimed / 100).saturating_add(10)
    }
}
