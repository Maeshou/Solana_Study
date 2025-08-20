use anchor_lang::prelude::*;

// ============================================================================
// 1) Prism Studio — 色合成スタジオ（PDAなし / has_one + tag不一致）
// ============================================================================
declare_id!("PRSM11111111111111111111111111111111111111111");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum StudioMode { Idle, Blend, Seal }

#[program]
pub mod prism_studio {
    use super::*;
    use StudioMode::*;

    pub fn init_studio(ctx: Context<InitStudio>, limit: u32) -> Result<()> {
        let s = &mut ctx.accounts;
        s.studio.owner = s.owner.key();
        s.studio.limit = limit;
        s.studio.mode = Idle;

        // 子口座の所属と一意タグを設定（A=1, B=2, Log=9）
        s.panel_a.studio = s.studio.key();
        s.panel_a.tag = 1;
        s.panel_b.studio = s.studio.key();
        s.panel_b.tag = 2;
        s.log.studio = s.studio.key();
        s.log.tag = 9;
        Ok(())
    }

    pub fn mix(ctx: Context<Mix>, steps: u32) -> Result<()> {
        let s = &mut ctx.accounts;

        for i in 0..steps {
            // クリップつき加算（u32::MAXで飽和）
            s.panel_a.hue = s.panel_a.hue.checked_add(3 + (i % 5)).unwrap_or(u32::MAX);
            s.panel_b.hue = s.panel_b.hue.checked_add(5 + (i % 7)).unwrap_or(u32::MAX);
            // ログは整数比ブレンド
            let delta = ((s.panel_a.hue as u64 + s.panel_b.hue as u64) / 16) as u32;
            s.log.strokes = s.log.strokes.saturating_add(delta);
        }

        let total = s.panel_a.hue as u64 + s.panel_b.hue as u64;
        if total > s.studio.limit as u64 {
            s.studio.mode = Seal;
            s.log.flags = s.log.flags.saturating_add(2);
            s.panel_a.hue = s.panel_a.hue / 2 + 7;
            s.panel_b.hue = s.panel_b.hue / 2 + 9;
            msg!("sealed: flags+2, both panels damped");
        } else {
            s.studio.mode = Blend;
            s.log.strokes = s.log.strokes.saturating_add(11);
            s.panel_a.hue ^= 0x00FF_00FF;
            s.panel_b.hue = s.panel_b.hue.saturating_add(13);
            msg!("blend: strokes+11, hue xor/boost");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStudio<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub studio: Account<'info, Studio>,
    #[account(init, payer=payer, space=8+32+4+4)]
    pub panel_a: Account<'info, Panel>,
    #[account(init, payer=payer, space=8+32+4+4)]
    pub panel_b: Account<'info, Panel>,
    #[account(init, payer=payer, space=8+32+8+4)]
    pub log: Account<'info, MixLog>,
    #[account(mut)] pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Mix<'info> {
    #[account(mut, has_one=owner)]
    pub studio: Account<'info, Studio>,
    // 同一口座の二重渡しを tag 不一致で防止（同一アカウントなら tag も同一で不一致条件に引っかかる）
    #[account(mut, has_one=studio, constraint = panel_a.tag != panel_b.tag @ PsErr::Dup)]
    pub panel_a: Account<'info, Panel>,
    #[account(mut, has_one=studio, constraint = panel_b.tag != log.tag @ PsErr::Dup)]
    pub panel_b: Account<'info, Panel>,
    #[account(mut, has_one=studio)]
    pub log: Account<'info, MixLog>,
    pub owner: Signer<'info>,
}

#[account] pub struct Studio { pub owner: Pubkey, pub limit: u32, pub mode: StudioMode }
#[account] pub struct Panel { pub studio: Pubkey, pub tag: u8, pub hue: u32 }
#[account] pub struct MixLog { pub studio: Pubkey, pub tag: u8, pub strokes: u32, pub flags: u32 }
#[error_code] pub enum PsErr { #[msg("duplicate mutable account")] Dup }
