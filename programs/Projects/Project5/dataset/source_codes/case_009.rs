// ============================================================================
// 1) Beast Breeder — 親NFTのブリード管理（require_keys_neq!）
// ============================================================================
declare_id!("BBR31111111111111111111111111111111");
use anchor_lang::prelude::*;

#[program]
pub mod beast_breeder {
    use super::*;

    pub fn init_barn(ctx: Context<InitBarn>, gene_cap: u64) -> Result<()> {
        let barn = &mut ctx.accounts.barn;
        barn.keeper = ctx.accounts.keeper.key();
        barn.gene_cap = gene_cap;
        barn.active = true;

        let lab = &mut ctx.accounts.gene_lab;
        lab.rolls = 0;
        lab.power = 0;
        lab.bump = *ctx.bumps.get("gene_lab").unwrap();

        let pol = &mut ctx.accounts.policy;
        pol.min_mix = 10;
        pol.max_mix = 200;
        pol.strict = true;
        Ok(())
    }

    pub fn breed_attempt(ctx: Context<BreedAttempt>, eggs: u32) -> Result<()> {
        // Duplicate Mutable Account 排除（親A, 親B, 研究所の全組み合わせ）
        require_keys_neq!(ctx.accounts.parent_a.key(), ctx.accounts.parent_b.key(), BreedErr::Same);
        require_keys_neq!(ctx.accounts.parent_a.key(), ctx.accounts.gene_lab.key(), BreedErr::Same);
        require_keys_neq!(ctx.accounts.parent_b.key(), ctx.accounts.gene_lab.key(), BreedErr::Same);

        // ループ：疑似遺伝子混合
        let mut done = 0u32;
        while done < eggs {
            ctx.accounts.parent_a.mutation = ctx.accounts.parent_a.mutation.saturating_add(1);
            ctx.accounts.parent_b.mutation = ctx.accounts.parent_b.mutation.saturating_add(2);
            ctx.accounts.gene_lab.rolls = ctx.accounts.gene_lab.rolls.saturating_add(1);
            done += 1;
        }

        // 分岐：閾値で barn/policy を更新
        if (ctx.accounts.parent_a.mutation + ctx.accounts.parent_b.mutation) as u64 > ctx.accounts.barn.gene_cap {
            ctx.accounts.barn.active = false;
            ctx.accounts.policy.strict = true;
            ctx.accounts.gene_lab.power = ctx.accounts.gene_lab.power.saturating_add(5);
            msg!("cap exceeded; lab power boosted, barn paused");
        } else {
            ctx.accounts.barn.active = true;
            ctx.accounts.policy.strict = false;
            ctx.accounts.gene_lab.power = ctx.accounts.gene_lab.power.saturating_add(1);
            msg!("within cap; breeding continues, lab warmed");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBarn<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8 + 1)]
    pub barn: Account<'info, Barn>,
    #[account(init, payer = payer, space = 8 + 8 + 8 + 1, seeds = [b"lab", payer.key().as_ref()], bump)]
    pub gene_lab: Account<'info, GeneLab>,
    #[account(init, payer = payer, space = 8 + 4 + 4 + 1)]
    pub policy: Account<'info, BreedPolicy>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub keeper: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BreedAttempt<'info> {
    #[account(mut)]
    pub barn: Account<'info, Barn>,
    #[account(mut)]
    pub parent_a: Account<'info, BeastCard>,
    #[account(mut)]
    pub parent_b: Account<'info, BeastCard>,
    #[account(mut)]
    pub gene_lab: Account<'info, GeneLab>,
    #[account(mut)]
    pub policy: Account<'info, BreedPolicy>,
}

#[account] pub struct Barn { pub keeper: Pubkey, pub gene_cap: u64, pub active: bool }
#[account] pub struct BeastCard { pub dna: u64, pub tier: u8, pub mutation: u32 }
#[account] pub struct GeneLab { pub rolls: u64, pub power: u64, pub bump: u8 }
#[account] pub struct BreedPolicy { pub min_mix: u32, pub max_mix: u32, pub strict: bool }

#[error_code] pub enum BreedErr { #[msg("same account passed twice")] Same }
