// ============================================================================
// 2) Hatchery（孵化所）— PDA使用(seedsで一意化) + constraint 複数
// ============================================================================
declare_id!("HTCH22222222222222222222222222222222");

#[program]
pub mod hatchery {
    use super::*;

    pub fn init_hatchery(ctx: Context<InitHatchery>, cap: u64) -> Result<()> {
        ctx.accounts.nursery.keeper = ctx.accounts.keeper.key();
        ctx.accounts.nursery.cap = cap;
        ctx.accounts.nursery.active = true;

        ctx.accounts.gene_pool.rolls = 0;
        ctx.accounts.gene_pool.heat = 0;

        ctx.accounts.rules.min_mix = 5;
        ctx.accounts.rules.max_mix = 300;
        ctx.accounts.rules.strict = true;
        Ok(())
    }

    pub fn breed(ctx: Context<Breed>, eggs: u32) -> Result<()> {
        // 属性で nursery != gene_pool、parent_a != parent_b を担保
        // さらに function内で parent_* と pool の重複も明示排除
        require!(ctx.accounts.parent_a.key() != ctx.accounts.gene_pool.key(), HatchErr::Dup);
        require!(ctx.accounts.parent_b.key() != ctx.accounts.gene_pool.key(), HatchErr::Dup);

        let mut e = 0;
        while e < eggs {
            ctx.accounts.parent_a.mutation = ctx.accounts.parent_a.mutation.saturating_add(1);
            ctx.accounts.parent_b.mutation = ctx.accounts.parent_b.mutation.saturating_add(2);
            ctx.accounts.gene_pool.rolls = ctx.accounts.gene_pool.rolls.saturating_add(1);
            e += 1;
        }

        let total = (ctx.accounts.parent_a.mutation as u64).saturating_add(ctx.accounts.parent_b.mutation as u64);
        if total > ctx.accounts.nursery.cap {
            ctx.accounts.nursery.active = false;
            ctx.accounts.rules.strict = true;
            ctx.accounts.gene_pool.heat = ctx.accounts.gene_pool.heat.saturating_add(5);
            msg!("cap exceeded: {}", total);
        } else {
            ctx.accounts.nursery.active = true;
            ctx.accounts.rules.strict = false;
            ctx.accounts.gene_pool.heat = ctx.accounts.gene_pool.heat.saturating_add(1);
            msg!("within cap: {}", total);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitHatchery<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8 + 1, seeds=[b"nursery", payer.key().as_ref()], bump)]
    pub nursery: Account<'info, Nursery>,
    #[account(init, payer = payer, space = 8 + 8 + 8, seeds=[b"pool", payer.key().as_ref()], bump)]
    pub gene_pool: Account<'info, GenePool>,
    #[account(init, payer = payer, space = 8 + 4 + 4 + 1)]
    pub rules: Account<'info, BreedRules>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub keeper: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Breed<'info> {
    #[account(mut, constraint = nursery.key() != gene_pool.key(), error = HatchErr::Dup)]
    pub nursery: Account<'info, Nursery>,
    #[account(mut)]
    pub gene_pool: Account<'info, GenePool>,
    #[account(mut)]
    pub rules: Account<'info, BreedRules>,
    #[account(mut)]
    pub parent_a: Account<'info, Beast>,
    #[account(mut, constraint = parent_a.key() != parent_b.key(), error = HatchErr::Dup)]
    pub parent_b: Account<'info, Beast>,
}

#[account] pub struct Nursery { pub keeper: Pubkey, pub cap: u64, pub active: bool }
#[account] pub struct GenePool { pub rolls: u64, pub heat: u64 }
#[account] pub struct BreedRules { pub min_mix: u32, pub max_mix: u32, pub strict: bool }
#[account] pub struct Beast { pub dna: u64, pub tier: u8, pub mutation: u32 }

#[error_code] pub enum HatchErr { #[msg("dup")] Dup }

