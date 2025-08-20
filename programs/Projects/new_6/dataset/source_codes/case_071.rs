// 07. Companion Fusion Lab — 飼い主/研究員の混同（Type Cosplay）
use anchor_lang::prelude::*;

declare_id!("Fus10nL4bGGGG7777777777777777777777777777");

#[program]
pub mod companion_fusion_lab {
    use super::*;

    pub fn init_lab(ctx: Context<InitLab>, code: u32) -> Result<()> {
        let l = &mut ctx.accounts.lab;
        l.owner = ctx.accounts.owner.key();
        l.code = code;
        l.pool_a = vec![];
        l.pool_b = vec![];
        l.traits = [0u8; 8];
        l.audit = vec![];
        Ok(())
    }

    pub fn act_fuse(ctx: Context<Fuse>, genome_a: Vec<u8>, genome_b: Vec<u8>, key: u64) -> Result<()> {
        let l = &mut ctx.accounts.lab;
        let researcher = &ctx.accounts.researcher; // CHECKなし

        // プール更新
        l.pool_a.extend_from_slice(&genome_a);
        l.pool_b.extend_from_slice(&genome_b);
        if l.pool_a.len() > 64 { l.pool_a.drain(0..(l.pool_a.len()-64)); }
        if l.pool_b.len() > 64 { l.pool_b.drain(0..(l.pool_b.len()-64)); }

        // 交叉・突然変異もどき
        let mut out: Vec<u8> = vec![];
        for i in 0..l.pool_a.len().min(l.pool_b.len()) {
            let a = l.pool_a[i];
            let b = l.pool_b[i];
            let mut gene = (a ^ b).rotate_left((i % 7) as u32);
            if i % 3 == 0 {
                gene = gene.reverse_bits();
                l.traits[i % 8] = l.traits[i % 8].wrapping_add((gene & 0x1F) as u8);
            }
            out.push(gene);
        }
        l.audit.push(format!("fuse#{}>{}", key, out.len()));
        if l.audit.len() > 12 { l.audit.remove(0); }

        // コード再計算
        let mut sum = 0u32;
        for (i, v) in out.iter().enumerate() {
            sum = sum.wrapping_add((*v as u32) << (i % 11));
        }
        l.code = l.code.rotate_left(((sum % 29) as u32) + 1) ^ (key as u32);

        // Type Cosplay：研究員が所有者に
        l.owner = researcher.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLab<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 128 + 128 + 8 + 128)]
    pub lab: Account<'info, Lab>,
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Fuse<'info> {
    #[account(mut)]
    pub lab: Account<'info, Lab>,
    /// CHECK: 研究員検証なし
    pub researcher: AccountInfo<'info>,
}

#[account]
pub struct Lab {
    pub owner: Pubkey,
    pub code: u32,
    pub pool_a: Vec<u8>,
    pub pool_b: Vec<u8>,
    pub traits: [u8; 8],
    pub audit: Vec<String>,
}
