// ============================================================================
// 3) Warehouse Ops（倉庫搬送） — assert_ne! + has_one
// ============================================================================
declare_id!("WH33333333333333333333333333333333");

#[program]
pub mod warehouse_ops {
    use super::*;

    pub fn init_bin(ctx: Context<InitBin>, slot: u32) -> Result<()> {
        ctx.accounts.bin.owner = ctx.accounts.manager.key();
        ctx.accounts.bin.slot = slot;
        ctx.accounts.bin.count = 0;

        ctx.accounts.log.moves = 0;
        ctx.accounts.log.ok = true;
        ctx.accounts.log.bump = *ctx.bumps.get("log").unwrap();

        ctx.accounts.policy.max_per_move = 50;
        ctx.accounts.policy.strict = true;
        ctx.accounts.policy.version = 1;
        Ok(())
    }

    pub fn move_goods(ctx: Context<MoveGoods>, qty: u32) -> Result<()> {
        // 三つ巴のうち少なくとも bin != log, bin != policy は型・属性で独立
        assert_ne!(ctx.accounts.log.key(), ctx.accounts.policy.key(), "accounts must differ");

        let mut moved = 0u32;
        while moved < qty {
            ctx.accounts.bin.count = ctx.accounts.bin.count.saturating_add(1);
            ctx.accounts.log.moves = ctx.accounts.log.moves.saturating_add(1);
            moved += 1;
        }

        if qty > ctx.accounts.policy.max_per_move {
            ctx.accounts.policy.version = ctx.accounts.policy.version.saturating_add(1);
            ctx.accounts.log.ok = false;
            msg!("excess move: qty={} limit={}", qty, ctx.accounts.policy.max_per_move);
            ctx.accounts.bin.slot = ctx.accounts.bin.slot.saturating_add(1);
        } else {
            ctx.accounts.log.ok = true;
            ctx.accounts.policy.strict = true;
            msg!("move ok: count={} moves={}", ctx.accounts.bin.count, ctx.accounts.log.moves);
            ctx.accounts.bin.slot = ctx.accounts.bin.slot.saturating_add(0);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBin<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 8)]
    pub bin: Account<'info, BinRecord>,
    #[account(init, seeds = [b"log", payer.key().as_ref()], bump, payer = payer, space = 8 + 4 + 1 + 1)]
    pub log: Account<'info, MoveLog>,
    #[account(init, payer = payer, space = 8 + 4 + 1 + 4)]
    pub policy: Account<'info, Policy>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MoveGoods<'info> {
    #[account(mut, has_one = owner)]
    pub bin: Account<'info, BinRecord>,
    #[account(mut)]
    pub log: Account<'info, MoveLog>,
    #[account(mut)]
    pub policy: Account<'info, Policy>,
    pub owner: Signer<'info>,
}

#[account] pub struct BinRecord { pub owner: Pubkey, pub slot: u32, pub count: u64 }
#[account] pub struct MoveLog { pub moves: u32, pub ok: bool, pub bump: u8 }
#[account] pub struct Policy { pub max_per_move: u32, pub strict: bool, pub version: u32 }

