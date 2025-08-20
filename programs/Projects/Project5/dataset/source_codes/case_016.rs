// ============================================================================
// 1) Dragon Nursery（ドラゴン孵化）— PDA使用（bumpは保存しない）+ constraint + has_one
//    防止法: seeds一意化, has_one, 属性constraint（親A≠親B, 各ペア≠研究所）
// ============================================================================
declare_id!("DRGN11111111111111111111111111111111");
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum NurseryState { Paused, Active }

#[program]
pub mod dragon_nursery {
    use super::*;

    pub fn init_nursery(ctx: Context<InitNursery>, cap: u64) -> Result<()> {
        let n = &mut ctx.accounts.nursery;
        n.warden = ctx.accounts.warden.key();
        n.cap = cap;
        n.state = NurseryState::Active;

        let lab = &mut ctx.accounts.lab;
        lab.mixes = 0;
        lab.heat = 0;

        let pol = &mut ctx.accounts.policy;
        pol.min_mix = 8;
        pol.max_mix = 256;
        pol.strict = 1; // 0:柔, 1:厳
        Ok(())
    }

    pub fn mix_eggs(ctx: Context<MixEggs>, batches: u32) -> Result<()> {
        // ループ
        let mut i = 0;
        while i < batches {
            ctx.accounts.parent_a.mut_points = ctx.accounts.parent_a.mut_points.saturating_add(1);
            ctx.accounts.parent_b.mut_points = ctx.accounts.parent_b.mut_points.saturating_add(2);
            ctx.accounts.lab.mixes = ctx.accounts.lab.mixes.saturating_add(1);
            i += 1;
        }

        // 分岐（各ブロック4行以上）
        let total = (ctx.accounts.parent_a.mut_points as u64).saturating_add(ctx.accounts.parent_b.mut_points as u64);
        if total > ctx.accounts.nursery.cap {
            ctx.accounts.nursery.state = NurseryState::Paused;
            ctx.accounts.policy.strict = 1;
            ctx.accounts.lab.heat = ctx.accounts.lab.heat.saturating_add(5);
            msg!("cap exceeded: total={} cap={}", total, ctx.accounts.nursery.cap);
        } else {
            ctx.accounts.nursery.state = NurseryState::Active;
            ctx.accounts.policy.strict = 0;
            ctx.accounts.lab.heat = ctx.accounts.lab.heat.saturating_add(1);
            msg!("within cap: total={} cap={}", total, ctx.accounts.nursery.cap);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitNursery<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8 + 1, seeds=[b"nursery", payer.key().as_ref()], bump)]
    pub nursery: Account<'info, Nursery>,
    #[account(init, payer = payer, space = 8 + 8 + 8, seeds=[b"lab", payer.key().as_ref()], bump)]
    pub lab: Account<'info, Lab>,
    #[account(init, payer = payer, space = 8 + 4 + 4 + 1)]
    pub policy: Account<'info, MixPolicy>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub warden: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MixEggs<'info> {
    #[account(mut, has_one = warden)]
    pub nursery: Account<'info, Nursery>,
    #[account(mut)]
    pub lab: Account<'info, Lab>,
    #[account(mut)]
    pub policy: Account<'info, MixPolicy>,
    // 同型を2つ受けるので属性で相互に≠を保証
    #[account(mut, constraint = parent_a.key() != parent_b.key(), error = EggErr::Same)]
    pub parent_a: Account<'info, DragonCard>,
    #[account(mut, constraint = parent_b.key() != lab.key(), error = EggErr::Same)]
    pub parent_b: Account<'info, DragonCard>,
    pub warden: Signer<'info>,
}

#[account] pub struct Nursery { pub warden: Pubkey, pub cap: u64, pub state: NurseryState }
#[account] pub struct Lab { pub mixes: u64, pub heat: u64 }
#[account] pub struct MixPolicy { pub min_mix: u32, pub max_mix: u32, pub strict: u8 }
#[account] pub struct DragonCard { pub dna: u64, pub tier: u8, pub mut_points: u32 }

#[error_code] pub enum EggErr { #[msg("same account passed twice")] Same }
