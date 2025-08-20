// (9) Inventory Workshop — 在庫と工房（入出庫スロット・記録）
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod inventory_workshop {
    use super::*;
    use SlotType::*;

    pub fn init_warehouse(ctx: Context<InitWarehouse>, code: u32) -> Result<()> {
        let w = &mut ctx.accounts.warehouse;
        w.owner = ctx.accounts.owner.key();
        w.code = code;
        w.items = 0;
        Ok(())
    }

    pub fn create_slots(ctx: Context<CreateSlots>, input_kind: SlotType, output_kind: SlotType) -> Result<()> {
        let w = &mut ctx.accounts.warehouse;
        let a = &mut ctx.accounts.input_slot;
        a.warehouse = w.key();
        a.kind = input_kind;
        a.count = 0;
        let b = &mut ctx.accounts.output_slot;
        b.warehouse = w.key();
        b.kind = output_kind;
        b.count = 0;
        Ok(())
    }

    pub fn move_items(ctx: Context<MoveItems>, batches: Vec<u16>) -> Result<()> {
        let w = &mut ctx.accounts.warehouse;
        let input = &mut ctx.accounts.input_slot;
        let output = &mut ctx.accounts.output_slot;
        let rec = &mut ctx.accounts.record;

        let mut moved: u32 = 0;
        let mut entropy: u16 = 0;
        for b in batches {
            let cap = b.min(1000);
            moved = moved.saturating_add(cap as u32);
            entropy ^= (cap << 1) | (cap >> 1);
        }

        if input.kind == Inlet {
            let take = moved.min(input.count as u32);
            input.count = input.count.saturating_sub(take as u16);
            output.count = output.count.saturating_add((take as u16).min(2000));
            w.items = w.items.saturating_add(take);
            msg!("Inlet path: moved={}, in.count={}, out.count={}", moved, input.count, output.count);
        } else {
            let give = moved.min(2000);
            input.count = input.count.saturating_add(give as u16);
            output.count = output.count.saturating_sub(give.min(output.count as u32) as u16);
            w.items = w.items.saturating_add(give);
            msg!("Outlet path: moved={}, in.count={}, out.count={}", moved, input.count, output.count);
        }

        // sqrt 近似で熱量メトリクス
        let mut x = (w.items as u128).max(1);
        let mut i = 0;
        while i < 3 {
            x = (x + (w.items as u128 / x)).max(1) / 2;
            i += 1;
        }
        rec.warehouse = w.key();
        rec.heat = (x as u32).saturating_add(entropy as u32);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWarehouse<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 4)]
    pub warehouse: Account<'info, Warehouse>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateSlots<'info> {
    #[account(mut)]
    pub warehouse: Account<'info, Warehouse>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 2)]
    pub input_slot: Account<'info, SlotCard>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 2)]
    pub output_slot: Account<'info, SlotCard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 同一倉庫 + スロット種類不一致
#[derive(Accounts)]
pub struct MoveItems<'info> {
    #[account(mut)]
    pub warehouse: Account<'info, Warehouse>,
    #[account(mut, has_one = warehouse)]
    pub record: Account<'info, Record>,
    #[account(
        mut,
        has_one = warehouse,
        constraint = input_slot.kind != output_slot.kind @ ErrCode::CosplayBlocked
    )]
    pub input_slot: Account<'info, SlotCard>,
    #[account(mut, has_one = warehouse)]
    pub output_slot: Account<'info, SlotCard>,
}

#[account]
pub struct Warehouse {
    pub owner: Pubkey,
    pub code: u32,
    pub items: u32,
}

#[account]
pub struct SlotCard {
    pub warehouse: Pubkey,
    pub kind: SlotType,
    pub count: u16,
}

#[account]
pub struct Record {
    pub warehouse: Pubkey,
    pub heat: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum SlotType {
    Inlet,
    Outlet,
}

#[error_code]
pub enum ErrCode {
    #[msg("Type cosplay prevented in inventory move.")]
    CosplayBlocked,
}
