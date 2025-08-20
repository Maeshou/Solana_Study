// ============================================================================
// 7) Glyph Bakery — 焼成（固定小数点Q32.32/端数バッファ）— PDAあり
// ============================================================================
declare_id!("GLBK777777777777777777777777777777777");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum OvenPhase { Prep, Bake, Rest }

#[program]
pub mod glyph_bakery {
    use super::*;

    pub fn init_oven(ctx: Context<InitOven>, temp_q32: u64) -> Result<()> {
        let b = &mut ctx.accounts;
        b.oven.baker = b.baker.key();
        b.oven.temp_q32 = temp_q32; // Q32.32
        b.oven.phase = OvenPhase::Prep;
        Ok(())
    }

    pub fn bake(ctx: Context<Bake>, batches: u32) -> Result<()> {
        let b = &mut ctx.accounts;

        for _ in 0..batches {
            // rise_q32 = rise_q32 + temp_q32 * 0.015625 (1/64)
            let inc = (u128::from(b.oven.temp_q32) >> 6) + u128::from(b.frac_remainder);
            let new = u128::from(b.dough.rise_q32) + inc;
            b.dough.rise_q32 = (new & ((1u128<<64)-1)) as u64;
            b.frac_remainder = ((new >> 64) as u32).wrapping_add(0); // 端数を次回へ

            // crust: ビットスワップミキサ
            let c = b.dough.crust.rotate_left(7) ^ b.dough.crust.rotate_right(3);
            b.dough.crust = c.wrapping_mul(0x9E37_79B9);
        }

        let heat = (b.oven.temp_q32 >> 32) as u32;
        if heat > 220 {
            b.oven.phase = OvenPhase::Rest;
            b.log.batches = b.log.batches.wrapping_add(1);
            b.dough.crust = b.dough.crust ^ 0xDEAD_BEEF;
            msg!("rest: batches++, crust xor");
        } else {
            b.oven.phase = OvenPhase::Bake;
            b.log.charms = b.log.charms.wrapping_add(2);
            b.dough.rise_q32 = b.dough.rise_q32.wrapping_add(1<<32);
            msg!("bake: charms+2, rise+1.0");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitOven<'info> {
    #[account(init, payer=payer, space=8+32+8+1, seeds=[b"oven", baker.key().as_ref()], bump)]
    pub oven: Account<'info, OvenCfg>,
    #[account(init, payer=payer, space=8+8+4)]
    pub dough: Account<'info, Dough>,
    #[account(init, payer=payer, space=8+4+4)]
    pub log: Account<'info, BakeLog>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub baker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Bake<'info> {
    #[account(mut, seeds=[b"oven", baker.key().as_ref()], bump, has_one=baker)]
    pub oven: Account<'info, OvenCfg>,
    #[account(mut, constraint = oven.key() != dough.key(), error = BakeErr::Dup)]
    pub dough: Account<'info, Dough>,
    #[account(mut)]
    pub log: Account<'info, BakeLog>,
    pub baker: Signer<'info>,
    /// CHECK: 端数用バッファ保持のためプログラム内部でのみ使用（例示）
    // 実運用ではPDAやアカウントに持たせる
    // ここでは簡略のため context構造体に含める変数とする（デモ用）
}

#[account] pub struct OvenCfg { pub baker: Pubkey, pub temp_q32: u64, pub phase: OvenPhase }
#[account] pub struct Dough { pub rise_q32: u64, pub crust: u32 }
#[account] pub struct BakeLog { pub batches: u32, pub charms: u32 }

// 端数バッファ（簡易デモ用）：実運用なら別アカウント化を推奨
#[derive(Accounts)]
pub struct Dummy<'info> { }

#[error_code] pub enum BakeErr { #[msg("dup")] Dup }

