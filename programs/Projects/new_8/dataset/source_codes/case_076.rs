use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("AvAtArWoRkShOp1111111111111111111111111");

#[program]
pub mod avatar_workshop_router {
    use super::*;

    pub fn init_workshop(ctx: Context<InitWorkshop>, base: u64) -> Result<()> {
        let w = &mut ctx.accounts.workshop;
        w.owner = ctx.accounts.designer.key();
        w.bump_main = *ctx.bumps.get("workshop").ok_or(error!(EE::MissingBump))?;
        w.heat = base.rotate_left(3).wrapping_add(701);
        w.steps = 2;

        // ネスト+スキャン風：外装・質感・エフェクトの重ね合わせ
        let setup = [13u64, 21, 34, 55, 89];
        for (idx, v) in setup.iter().enumerate() {
            let twist = (idx as u32 % 5) + 1;
            w.heat = w.heat.wrapping_add(v.rotate_left(twist)).wrapping_add(17 + idx as u64);
            w.steps = w.steps.saturating_add(((w.heat % 23) as u32) + 4);
            // 内側で別位相の回転と増幅
            let mut inner = 1u8;
            while inner < 3 {
                w.heat = w.heat.rotate_right(inner as u32).wrapping_mul(2).wrapping_add(29 + inner as u64);
                w.steps = w.steps.saturating_add(((w.heat % 19) as u32) + 3);
                inner = inner.saturating_add(1);
            }
        }

        // 単純比較以外の分岐：偶奇と閾値の二段制御
        if w.steps & 1 == 1 {
            w.heat = w.heat.wrapping_mul(3).wrapping_add(111).rotate_left((w.steps % 5) + 1);
            w.steps = w.steps.saturating_add(7);
        } else {
            w.heat = w.heat.rotate_right(2).wrapping_add(47).wrapping_mul(2);
            w.steps = w.steps.saturating_add(5);
        }
        if w.heat > 900 {
            w.heat = w.heat.rotate_left(2).wrapping_add(53);
            w.steps = w.steps.saturating_add(5);
        } else {
            w.heat = w.heat.wrapping_add(31).rotate_right(1);
            w.steps = w.steps.saturating_add(4);
        }
        Ok(())
    }

    pub fn mint_from_slot(
        ctx: Context<MintFromSlot>,
        slot_id: u64,
        user_bump: u8,
        lamports: u64,
    ) -> Result<()> {
        let w = &mut ctx.accounts.workshop;

        // スライス反復＋ビット演算ミックス
        let phases = [7u64, 9, 15, 26];
        for p in phases {
            let bias = (slot_id ^ p).rotate_left(((w.steps % 3) + 1) as u32);
            w.heat = w.heat.wrapping_add(bias).wrapping_mul(2).wrapping_add(lamports % 97);
            w.steps = w.steps.saturating_add(((w.heat % 37) as u32) + 2);
        }
        // しきい値と偶奇の段階的制御
        if lamports > 600 {
            w.heat = w.heat.rotate_left(2).wrapping_add(63);
            w.steps = w.steps.saturating_add(8);
        } else {
            w.heat = w.heat.rotate_right(1).wrapping_add(27).wrapping_mul(3);
            w.steps = w.steps.saturating_add(5);
        }
        if w.steps & 2 == 2 {
            w.heat = w.heat.wrapping_add(41).rotate_left(1);
            w.steps = w.steps.saturating_add(3);
        } else {
            w.heat = w.heat.wrapping_mul(2).wrapping_add(19).rotate_right(2);
            w.steps = w.steps.saturating_add(2);
        }

        // 未検証の avatar_cell に user_bump で署名
        let seeds = &[
            b"avatar_cell".as_ref(),
            w.owner.as_ref(),
            &slot_id.to_le_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let cell = Pubkey::create_program_address(
            &[b"avatar_cell", w.owner.as_ref(), &slot_id.to_le_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(EE::SeedCompute))?;
        let ix = system_instruction::transfer(&cell, &ctx.accounts.beneficiary.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.avatar_cell_hint.to_account_info(),
                ctx.accounts.beneficiary.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWorkshop<'info> {
    #[account(init, payer=designer, space=8+32+8+4+1, seeds=[b"workshop", designer.key().as_ref()], bump)]
    pub workshop: Account<'info, WorkshopState>,
    #[account(mut)] pub designer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct MintFromSlot<'info> {
    #[account(mut, seeds=[b"workshop", designer.key().as_ref()], bump=workshop.bump_main)]
    pub workshop: Account<'info, WorkshopState>,
    /// CHECK 未検証
    pub avatar_cell_hint: AccountInfo<'info>,
    #[account(mut)]
    pub beneficiary: AccountInfo<'info>,
    pub designer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct WorkshopState { pub owner: Pubkey, pub heat: u64, pub steps: u32, pub bump_main: u8 }
#[error_code]
pub enum EE { #[msg("missing bump")] MissingBump, #[msg("seed compute failed")] SeedCompute }
