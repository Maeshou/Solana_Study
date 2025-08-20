// ============================================================================
// 2) Rune Loom — ルーン織機（LCG/モジュロ/ビットマスク）— PDAなし
// ============================================================================
declare_id!("RLOM222222222222222222222222222222222");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LoomPhase { Warp, Weft, Rest }

#[program]
pub mod rune_loom {
    use super::*;

    pub fn init_loom(ctx: Context<InitLoom>, modulo: u32) -> Result<()> {
        let p = &mut ctx.accounts;
        p.loom.weaver = p.weaver.key();
        p.loom.modulo = modulo.max(17); // 最低値確保
        p.loom.phase = LoomPhase::Warp;
        Ok(())
    }

    pub fn weave(ctx: Context<Weave>, steps: u32) -> Result<()> {
        let p = &mut ctx.accounts;
        assert_ne!(p.loom.key(), p.log.key(), "loom/log must differ");

        for k in 0..steps {
            // LCG 位置更新
            p.sheet.pos = p.sheet.pos.wrapping_mul(1664525).wrapping_add(1013904223);

            // モジュロでパターン抽出（bitmaskも混ぜる）
            let pat = (p.sheet.pos ^ p.sheet.pos.rotate_left(9)) & 0x00FF_FFFF;
            let inc = (pat as u64 % p.loom.modulo as u64) + (k as u64 % 3);
            p.sheet.weight = (u128::from(p.sheet.weight)
                + u128::from(inc) * 3u128)
                .min(u128::from(u64::MAX)) as u64;

            p.log.rows = p.log.rows.wrapping_add((pat & 0xFF) as u32);
        }

        if p.sheet.weight % 2 == 0 {
            p.loom.phase = LoomPhase::Weft;
            p.log.flags = p.log.flags.wrapping_add(5);
            p.sheet.pos ^= 0x5A5A_A5A5;
            msg!("even weight: Weft, flags+=5, pos xored");
        } else {
            p.loom.phase = LoomPhase::Rest;
            p.log.rows = p.log.rows.wrapping_mul(2).wrapping_add(1);
            p.sheet.weight = p.sheet.weight / 2 + 7;
            msg!("odd weight: Rest, rows=2x+1, weight halved+7");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLoom<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub loom: Account<'info, Loom>,
    #[account(init, payer=payer, space=8+4+8)]
    pub sheet: Account<'info, Sheet>,
    #[account(init, payer=payer, space=8+4+4)]
    pub log: Account<'info, LoomLog>,
    #[account(mut)] pub payer: Signer<'info>,
    pub weaver: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Weave<'info> {
    #[account(mut, has_one=weaver, constraint = loom.key() != sheet.key(), error = LoomErr::Dup)]
    pub loom: Account<'info, Loom>,
    #[account(mut, constraint = sheet.key() != log.key(), error = LoomErr::Dup)]
    pub sheet: Account<'info, Sheet>,
    #[account(mut, constraint = loom.key() != log.key(), error = LoomErr::Dup)]
    pub log: Account<'info, LoomLog>,
    pub weaver: Signer<'info>,
}

#[account] pub struct Loom { pub weaver: Pubkey, pub modulo: u32, pub phase: LoomPhase }
#[account] pub struct Sheet { pub pos: u32, pub weight: u64 }
#[account] pub struct LoomLog { pub rows: u32, pub flags: u32 }
#[error_code] pub enum LoomErr { #[msg("dup")] Dup }