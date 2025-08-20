// ===============================================
// (8) craft_stock: 在庫/工房（レシピ・キュー・倉庫）
//   - 多層防御: has_one + recipe.tag 不一致
// ===============================================
use anchor_lang::prelude::*;
declare_id!("CrAfTsTok8888888888888888888888888888888");

#[program]
pub mod craft_stock {
    use super::*;

    pub fn init_recipe(ctx: Context<InitRecipe>, tag: u16) -> Result<()> {
        let r = &mut ctx.accounts.recipe;
        r.owner = ctx.accounts.owner.key();
        r.tag = tag;
        r.ratio = 50;
        Ok(())
    }

    pub fn init_queue(ctx: Context<InitQueue>) -> Result<()> {
        let q = &mut ctx.accounts.queue;
        q.parent = ctx.accounts.recipe.key();
        q.head = 0;
        q.tail = 0;
        Ok(())
    }

    pub fn init_warehouse2(ctx: Context<InitWarehouse2>) -> Result<()> {
        let w = &mut ctx.accounts.warehouse2;
        w.owner = ctx.accounts.owner.key();
        w.good = 0;
        w.bad = 0;
        w.hash = 0;
        Ok(())
    }

    /// 2つのレシピの tag が一致しないことを要求（Type Cosplay抑止）
    pub fn process_batch(ctx: Context<ProcessBatch>, cycles: u16) -> Result<()> {
        let ra = &mut ctx.accounts.recipe_a;
        let rb = &mut ctx.accounts.recipe_b;
        let qa = &mut ctx.accounts.queue_a;
        let qb = &mut ctx.accounts.queue_b;
        let w = &mut ctx.accounts.warehouse2;

        let mut i = 0u16;
        while i < cycles {
            // キューから擬似 pop/push（頭と尻だけで旋回）
            qa.head = qa.head.wrapping_add(ra.ratio as u64 & 7);
            qb.tail = qb.tail.wrapping_add(rb.ratio as u64 & 11);

            // 生産/不良の決定
            let score = ((qa.head ^ qb.tail) as u32) & 0xFFFF;
            if (score & 1) == 0 {
                w.good = w.good.saturating_add((score / 3) as u64 + 1);
                ra.ratio = (ra.ratio + 1).min(100);
            } else {
                w.bad = w.bad.saturating_add((score / 2) as u64 + 1);
                rb.ratio = rb.ratio.saturating_sub(1).max(1);
            }
            // ハッシュ更新
            w.hash ^= ((ra.tag as u64) << 16) ^ ((rb.tag as u64) << 1) ^ (score as u64);

            i += 1;
        }
        Ok(())
    }
}

// -------------------- Accounts --------------------

#[derive(Accounts)]
pub struct InitRecipe<'info> {
    #[account(
        init,
        payer = owner,
        // 8 + 32(owner) + 2(tag) + 1(ratio)
        space = 8 + 32 + 2 + 1
    )]
    pub recipe: Account<'info, Recipe>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitQueue<'info> {
    #[account(mut)]
    pub recipe: Account<'info, Recipe>,
    #[account(
        init,
        payer = owner,
        // 8 + 32(parent) + 8(head) + 8(tail)
        space = 8 + 32 + 8 + 8
    )]
    pub queue: Account<'info, Queue>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitWarehouse2<'info> {
    #[account(
        init,
        payer = owner,
        // 8 + 32(owner) + 8(good) + 8(bad) + 8(hash)
        space = 8 + 32 + 8 + 8 + 8
    )]
    pub warehouse2: Account<'info, Warehouse2>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessBatch<'info> {
    #[account(mut)]
    pub recipe_a: Account<'info, Recipe>,
    #[account(mut)]
    pub recipe_b: Account<'info, Recipe>,
    #[account(
        mut,
        constraint = queue_a.parent == recipe_a.key() @ CraftErr::Cosplay
    )]
    pub queue_a: Account<'info, Queue>,
    #[account(
        mut,
        constraint = queue_b.parent == recipe_b.key() @ CraftErr::Cosplay,
        constraint = recipe_a.tag != recipe_b.tag @ CraftErr::Cosplay
    )]
    pub queue_b: Account<'info, Queue>,
    #[account(mut)]
    pub warehouse2: Account<'info, Warehouse2>,
}

// -------------------- Data --------------------

#[account]
pub struct Recipe {
    pub owner: Pubkey,
    pub tag: u16,
    pub ratio: u8,
}

#[account]
pub struct Queue {
    pub parent: Pubkey, // = recipe
    pub head: u64,
    pub tail: u64,
}

#[account]
pub struct Warehouse2 {
    pub owner: Pubkey,
    pub good: u64,
    pub bad: u64,
    pub hash: u64,
}

#[error_code]
pub enum CraftErr { #[msg("cosplay blocked")] Cosplay }
