// =============================================================================
// (2) Blacksmith Forge: 装備強化と耐久管理（Duplicate Mutable Account防止）
// =============================================================================
use anchor_lang::prelude::*;

declare_id!("FRG222222222222222222222222222222222222222");

#[program]
pub mod blacksmith_forge {
    use super::*;
    use SlotState::*;
    pub fn init_smithy(ctx: Context<InitSmithy>, name: String) -> Result<()> {
        let s = &mut ctx.accounts.smithy;
        s.master = ctx.accounts.master.key();
        s.name = name;
        s.capacity = 12;
        s.open = true;
        Ok(())
    }
    pub fn init_gear(ctx: Context<InitGear>, kind: u8, seed: u64) -> Result<()> {
        let g = &mut ctx.accounts.gear;
        g.parent = ctx.accounts.smithy.key();
        g.kind = kind;
        g.power = (seed & 0xFF) as u32;
        g.durability = 1000;
        g.state = Idle;
        g.locked = false;
        Ok(())
    }
    pub fn update_refine(ctx: Context<UpdateRefine>, cycles: u32) -> Result<()> {
        let a = &mut ctx.accounts.left;
        let b = &mut ctx.accounts.right;

        // ループ：三角波合成っぽくパワー偏移
        for t in 0..cycles.min(16) {
            let tri = (t as i64 % 8 - 4).abs() as u32;
            a.power = a.power.saturating_add(tri) - (tri / 2);
            b.power = b.power.saturating_add(3 * tri / 2) - tri / 3;
        }

        if a.power >= b.power {
            a.state = Busy;
            b.state = Idle;
            a.durability = a.durability.saturating_sub(7);
            b.durability = b.durability.checked_add(2).unwrap_or(u32::MAX);
            msg!("Left refines more, right cools down");
        } else {
            b.state = Busy;
            a.state = Idle;
            b.durability = b.durability.saturating_sub(7);
            a.durability = a.durability.checked_add(2).unwrap_or(u32::MAX);
            msg!("Right refines more, left cools down");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSmithy<'info> {
    #[account(init, payer = master, space = 8 + 32 + 4 + 1 + 32)]
    pub smithy: Account<'info, Smithy>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitGear<'info> {
    #[account(mut)]
    pub smithy: Account<'info, Smithy>,
    #[account(init, payer = owner, space = 8 + 32 + 1 + 4 + 4 + 1 + 1)]
    pub gear: Account<'info, Gear>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateRefine<'info> {
    pub smithy: Account<'info, Smithy>,
    #[account(mut, has_one = parent, constraint = left.key() != right.key())]
    pub left: Account<'info, Gear>,
    #[account(mut, has_one = parent)]
    pub right: Account<'info, Gear>,
    pub parent: Account<'info, Smithy>,
}

#[account]
pub struct Smithy {
    pub master: Pubkey,
    pub name: String,
    pub capacity: u8,
    pub open: bool,
}

#[account]
pub struct Gear {
    pub parent: Pubkey, // Smithy
    pub kind: u8,
    pub power: u32,
    pub durability: u32,
    pub state: SlotState,
    pub locked: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum SlotState { Idle, Busy }

#[error_code]
pub enum ForgeError {
    #[msg("Left and Right must differ")]
    SameGear,
}

