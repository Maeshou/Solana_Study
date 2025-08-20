// ============================================================================
// 2) Relic Atelier — 刻印工房（PDAなし・implメソッド・イテレータ）
//    防止: has_one + constraint三連
// ============================================================================
declare_id!("RLAT22222222222222222222222222222222");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Phase { Outline, Carve, Finish }

#[program]
pub mod relic_atelier {
    use super::*;

    pub fn init_studio(ctx: Context<InitStudio>, threshold: u32) -> Result<()> {
        let accs = &mut ctx.accounts;
        accs.studio.master = accs.master.key();
        accs.studio.threshold = threshold;
        accs.studio.phase = Phase::Outline;
        Ok(())
    }

    pub fn engrave(ctx: Context<Engrave>, strokes: u16) -> Result<()> {
        let a = &mut ctx.accounts;
        // implメソッドに実務を寄せる
        let impact = a.tablet.apply_strokes(strokes, &mut a.tools);
        a.log.entries = a.log.entries.saturating_add(impact as u64);

        if a.tablet.potency > a.studio.threshold as u64 {
            a.studio.phase = Phase::Finish;
            a.tools.file = a.tools.file.saturating_add(7);
            a.log.pages = a.log.pages.saturating_add(2);
            msg!("finish phase: file+7 pages+2");
        } else {
            a.studio.phase = Phase::Carve;
            a.tools.chisel = a.tools.chisel.saturating_add(5);
            a.log.entries = a.log.entries.saturating_add(3);
            msg!("carving: chisel+5 entries+3");
        }
        Ok(())
    }
}

impl Tablet {
    /// strokes を段階重みで反映（イテレータを使い、ctx直書きを減らす）
    fn apply_strokes(&mut self, strokes: u16, tools: &mut Tools) -> u32 {
        const WEIGHTS: [u32; 4] = [5, 7, 11, 13];
        let mut acc = 0u32;
        for (i, w) in WEIGHTS.iter().cycle().take(strokes as usize).enumerate() {
            self.runes = self.runes.saturating_add(*w);
            self.potency = self.potency.saturating_add((*w as u64) + (i as u64 % 3));
            tools.wear = tools.wear.saturating_add(2);
            acc = acc.saturating_add(*w);
        }
        acc
    }
}

#[derive(Accounts)]
pub struct InitStudio<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub studio: Account<'info, Studio>,
    #[account(init, payer = payer, space = 8 + 4 + 8)]
    pub tablet: Account<'info, Tablet>,
    #[account(init, payer = payer, space = 8 + 4 + 4 + 4)]
    pub tools: Account<'info, Tools>,
    #[account(init, payer = payer, space = 8 + 8 + 4)]
    pub log: Account<'info, StudioLog>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Engrave<'info> {
    #[account(mut, has_one = master, constraint = studio.key() != tablet.key(), error = RErr::Dup)]
    pub studio: Account<'info, Studio>,
    #[account(mut, constraint = tablet.key() != tools.key(), error = RErr::Dup)]
    pub tablet: Account<'info, Tablet>,
    #[account(mut, constraint = studio.key() != tools.key(), error = RErr::Dup)]
    pub tools: Account<'info, Tools>,
    #[account(mut)]
    pub log: Account<'info, StudioLog>,
    pub master: Signer<'info>,
}

#[account] pub struct Studio { pub master: Pubkey, pub threshold: u32, pub phase: Phase }
#[account] pub struct Tablet { pub runes: u32, pub potency: u64 }
#[account] pub struct Tools { pub chisel: u32, pub file: u32, pub wear: u32 }
#[account] pub struct StudioLog { pub entries: u64, pub pages: u32 }

#[error_code] pub enum RErr { #[msg("dup")] Dup }

