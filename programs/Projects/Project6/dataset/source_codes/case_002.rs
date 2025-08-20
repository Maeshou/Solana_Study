// ===============================================
// (2) gear_forge: 装備工房（設計図・製造ライン・在庫）
// ===============================================
use anchor_lang::prelude::*;
declare_id!("GeaRFoRgE2222222222222222222222222222222");

#[program]
pub mod gear_forge {
    use super::*;
    use Lane::*;

    pub fn init_blueprint(ctx: Context<InitBlueprint>, name: String, lane: Lane) -> Result<()> {
        let b = &mut ctx.accounts.blueprint;
        b.owner = ctx.accounts.owner.key();
        b.name = name;
        b.lane = lane;
        b.quality = 1;
        Ok(())
    }

    pub fn init_line(ctx: Context<InitLine>) -> Result<()> {
        let l = &mut ctx.accounts.line;
        let inv = &mut ctx.accounts.inventory;

        l.parent = ctx.accounts.blueprint.key();
        l.batch = 0;
        l.fail = 0;
        l.ok = 0;

        inv.parent = ctx.accounts.blueprint.key();
        inv.count = 0;
        inv.scrap = 0;
        inv.hash = 0;
        Ok(())
    }

    pub fn process_line(ctx: Context<ProcessLine>, cycles: u16) -> Result<()> {
        let bp = &mut ctx.accounts.blueprint;
        let line = &mut ctx.accounts.line;
        let inv = &mut ctx.accounts.inventory;

        let mut c = 0u16;
        while c < cycles {
            // ちいさなニュートン法（整数平方根の近似）
            let mut x = (bp.quality as u64).max(1);
            let mut k = 0;
            while k < 4 {
                x = (x + (bp.quality as u64).max(1) / x).max(1) / 2;
                k += 1;
            }
            let pass = ((x as u32) & 1) == 0;
            if pass {
                line.ok = line.ok.saturating_add(1);
                inv.count = inv.count.saturating_add(1);
                inv.hash ^= inv.count.rotate_left((inv.count % 31) as u32);
                bp.quality = (bp.quality + 1).min(100);
                msg!("produce ok: +1, inv={}, q={}", inv.count, bp.quality);
            } else {
                line.fail = line.fail.saturating_add(1);
                inv.scrap = inv.scrap.saturating_add(1);
                bp.quality = bp.quality.saturating_sub(1).max(1);
                msg!("produce fail: scrap+1, q={}", bp.quality);
            }
            line.batch = line.batch.wrapping_add(1);
            c += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBlueprint<'info> {
    #[account(
        init,
        payer = owner,
        // 8(discriminator) + 32(owner) + (4 + 64)(name) + 1(lane) + 1(quality)
        space = 8 + 32 + 4 + 64 + 1 + 1
    )]
    pub blueprint: Account<'info, Blueprint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitLine<'info> {
    #[account(mut, has_one = owner)]
    pub blueprint: Account<'info, Blueprint>,
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8)]
    pub line: Account<'info, Line>,
    // 在庫棚は固定アドレス（例）に pin：Type cosplay を address で遮断
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 8 + 8,
        address = inventory_fixed.key()
    )]
    pub inventory: Account<'info, Inventory>,
    /// CHECK: 固定先のアドレスを渡すダミー（実デプロイでは PDA/定数を想定）
    pub inventory_fixed: UncheckedAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessLine<'info> {
    #[account(mut, has_one = owner)]
    pub blueprint: Account<'info, Blueprint>,
    #[account(
        mut,
        has_one = parent
    )]
    pub line: Account<'info, Line>,
    #[account(
        mut,
        has_one = parent
    )]
    pub inventory: Account<'info, Inventory>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Blueprint {
    pub owner: Pubkey,
    pub name: String,
    pub lane: Lane,
    pub quality: u8,
}

#[account]
pub struct Line {
    pub parent: Pubkey, // = blueprint
    pub batch: u64,
    pub ok: u64,
    pub fail: u64,
}

#[account]
pub struct Inventory {
    pub parent: Pubkey, // = blueprint
    pub count: u64,
    pub scrap: u64,
    pub hash: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Lane { Melee, Ranged, Magic }

#[error_code]
pub enum ForgeErr {
    #[msg("cosplay")]
    Cosplay,
}
