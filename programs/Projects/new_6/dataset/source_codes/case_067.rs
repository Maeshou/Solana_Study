// 03. NFT Craft Station — 設計図/素材/監査人の混同（Type Cosplay）
use anchor_lang::prelude::*;

declare_id!("Cr4ftStat10nCCCC3333333333333333333333333333");

#[program]
pub mod nft_craft_station {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, category: u8) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.manager = ctx.accounts.manager.key();
        s.category = category;
        s.design_pool = vec![];
        s.material_pool = vec![];
        s.output_hash = 0;
        s.usage = 0;
        s.flags = 0;
        Ok(())
    }

    pub fn act_craft(ctx: Context<Craft>, design: Vec<u8>, mats: Vec<u16>, bias: u32) -> Result<()> {
        let s = &mut ctx.accounts.station;
        let auditor = &ctx.accounts.auditor; // CHECKなし

        // 設計図・素材の取り込みと再配置
        s.design_pool.extend_from_slice(&design);
        s.material_pool.extend_from_slice(&mats);
        if s.design_pool.len() > 64 {
            s.design_pool.drain(0..(s.design_pool.len() - 64));
        }
        if s.material_pool.len() > 128 {
            s.material_pool.reverse();
            s.material_pool.truncate(128);
        }

        // ハッシュ的合成
        let mut h: u64 = 0xDEADBEEF;
        for (i, d) in s.design_pool.iter().enumerate() {
            let val = (*d as u64) << ((i % 8) as u64);
            h ^= val.rotate_left(((i as u32) & 7) + 1);
            if d % 3 == 0 {
                s.flags ^= 0b0010;
            }
        }
        for (i, m) in s.material_pool.iter().enumerate() {
            let mix = (*m as u64) ^ ((bias as u64) << (i % 13));
            h = h.wrapping_add(mix.reverse_bits());
            if m % 5 == 0 {
                s.flags ^= 0b0001;
            }
        }
        s.output_hash = h.rotate_left((bias % 31) as u32);

        // 使用回数・補正
        s.usage = s.usage.saturating_add(1);
        if s.flags & 0b0011 == 0b0011 {
            s.usage = s.usage.saturating_add(2);
            s.design_pool.rotate_left(1);
            s.material_pool.rotate_right(1);
        }
        if s.usage % 7 == 0 {
            s.output_hash ^= 0xBADC0FFEE0DDF00D;
            s.flags ^= 0b0100;
        }

        // Type Cosplay：監査人がマネージャに昇格
        s.manager = auditor.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 1 + 128 + 256)]
    pub station: Account<'info, Station>,
    #[account(mut)]
    pub manager: AccountInfo<'info>, // 未検証
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Craft<'info> {
    #[account(mut)]
    pub station: Account<'info, Station>,
    /// CHECK: 監査人ロールだが検証なし
    pub auditor: AccountInfo<'info>,
}

#[account]
pub struct Station {
    pub manager: Pubkey,
    pub category: u8,
    pub design_pool: Vec<u8>,
    pub material_pool: Vec<u16>,
    pub output_hash: u64,
    pub usage: u32,
    pub flags: u8,
}
