// ======================================================================
// 5) Orchestra Session：座席 & セクション（初期化=テンポから席/セクション分配）
// ======================================================================
declare_id!("ORCH55555555555555555555555555555555555555");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Rehearsal { Setup, Play, Break }

#[program]
pub mod orchestra_session {
    use super::*;
    use Rehearsal::*;

    pub fn init_stage(ctx: Context<InitStage>, tempo: u16) -> Result<()> {
        let st = &mut ctx.accounts.stage;
        st.owner = ctx.accounts.conductor.key();
        st.tempo = tempo;
        st.status = Setup;

        let ch = &mut ctx.accounts.chair_a;
        let chb = &mut ctx.accounts.chair_b;
        let sc = &mut ctx.accounts.section_log;

        ch.parent = st.key();  ch.chair = (tempo % 8) as u8;  ch.score = (tempo as u32) * 2 + 5;
        chb.parent = st.key(); chb.chair = ((tempo / 3) % 8) as u8; chb.score = (tempo as u32) + 11;

        sc.parent = st.key(); sc.section = ((tempo / 5) % 8) as u8; sc.count = 0; sc.total = 0;
        Ok(())
    }

    pub fn play(ctx: Context<Play>, bars: u32) -> Result<()> {
        let st = &mut ctx.accounts.stage;
        let a = &mut ctx.accounts.chair_a;
        let b = &mut ctx.accounts.chair_b;
        let sc = &mut ctx.accounts.section_log;

        for i in 0..bars {
            // 逐次平均
            a.score = a.score.saturating_add((i % 5) as u32 + 1);
            b.score = b.score.checked_add((i % 7) as u32 + 2).unwrap_or(u32::MAX);
            sc.count = sc.count.saturating_add(1);
            sc.total = sc.total.saturating_add((a.score + b.score) as u64);
        }

        let mean = if sc.count == 0 { 0 } else { (sc.total / sc.count) as u32 };
        if mean > st.tempo as u32 {
            st.status = Break;
            a.chair ^= 0x1;
            b.chair = b.chair.saturating_add(1);
            sc.section = sc.section.saturating_add(1);
            msg!("break: chair tweak & section++");
        } else {
            st.status = Play;
            a.score = a.score.saturating_add(9);
            b.score = b.score / 2 + 7;
            sc.total ^= 0x0FF0_FF0F;
            msg!("play: score adjust & total flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStage<'info> {
    #[account(init, payer=payer, space=8 + 32 + 2 + 1)]
    pub stage: Account<'info, StageCore>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub chair_a: Account<'info, Chair>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub chair_b: Account<'info, Chair>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub section_log: Account<'info, SectionLog>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub conductor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut, has_one=owner)]
    pub stage: Account<'info, StageCore>,
    #[account(
        mut,
        has_one=stage,
        constraint = chair_a.chair != chair_b.chair @ OrchErr::Dup
    )]
    pub chair_a: Account<'info, Chair>,
    #[account(
        mut,
        has_one=stage,
        constraint = chair_b.chair != section_log.section @ OrchErr::Dup
    )]
    pub chair_b: Account<'info, Chair>,
    #[account(mut, has_one=stage)]
    pub section_log: Account<'info, SectionLog>,
    pub conductor: Signer<'info>,
}

#[account] pub struct StageCore { pub owner: Pubkey, pub tempo: u16, pub status: Rehearsal }
#[account] pub struct Chair { pub parent: Pubkey, pub chair: u8, pub score: u32 }
#[account] pub struct SectionLog { pub parent: Pubkey, pub section: u8, pub count: u64, pub total: u64 }

#[error_code] pub enum OrchErr { #[msg("duplicate mutable account")] Dup }
