// 10) Fishing Fest Dist — ポアソン近似（決定論シーケンス）PDAあり
declare_id!("FFDP10101010101010101010101010101010");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LakeState { Open, Busy, Closed }

#[program]
pub mod fishing_fest_dist {
    use super::*;
    use LakeState::*;

    pub fn init_lake(ctx: Context<InitLake>, cap: u32) -> Result<()> {
        let l = &mut ctx.accounts;
        l.lake.warden = l.warden.key();
        l.lake.cap = cap;
        l.lake.state = Open;
        Ok(())
    }

    pub fn cast(ctx: Context<Cast>, tries: u32) -> Result<()> {
        let l = &mut ctx.accounts;

        for t in 0..tries {
            let phase = (t % 8) as u32;
            if phase < 4 {
                l.stats.lam_q16 = l.stats.lam_q16.saturating_add(1<<14);
                l.card.casts = l.card.casts.wrapping_add(1);
                l.trace.notes = l.trace.notes.wrapping_add(1);
                l.trace.flags = l.trace.flags ^ 0x1;
            } else {
                let dec = (1<<14).min(l.stats.lam_q16);
                l.stats.lam_q16 = l.stats.lam_q16 - dec;
                l.card.casts = l.card.casts.wrapping_add(1);
                l.trace.notes = l.trace.notes.wrapping_add(1);
                l.trace.flags = l.trace.flags ^ 0x2;
            }

            // 期待値/分散の近似蓄積
            l.stats.count_q16 = l.stats.count_q16.saturating_add(l.stats.lam_q16);
            l.stats.var_q16 = l.stats.var_q16.saturating_add(l.stats.lam_q16 / 2);
        }

        let exp_catch = l.stats.count_q16 >> 16;
        if exp_catch > l.lake.cap {
            l.lake.state = Closed;
            l.card.trophies = l.card.trophies.wrapping_add(3);
            l.stats.count_q16 = (l.lake.cap as u32) << 16;
            l.trace.flags = l.trace.flags | 0x10;
            msg!("closed: trophies+3, clamp count, flag set");
        } else {
            l.lake.state = Busy;
            l.card.casts = l.card.casts + 7;
            l.stats.var_q16 = l.stats.var_q16 + (1<<12);
            l.trace.notes = l.trace.notes.wrapping_mul(2);
            msg!("busy: casts+7, var bump, notes*2");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLake<'info> {
    #[account(init, payer=payer, space=8+32+4+1, seeds=[b"lake", warden.key().as_ref()], bump)]
    pub lake: Account<'info, LakeCfg>,
    #[account(init, payer=payer, space=8+4+4)]
    pub stats: Account<'info, PoissonStats>,
    #[account(init, payer=payer, space=8+4+4, seeds=[b"card", warden.key().as_ref()], bump)]
    pub card: Account<'info, FishCard>,
    #[account(init, payer=payer, space=8+8+4)]
    pub trace: Account<'info, LakeTrace>,
    #[account(mut)] pub payer: Signer<'info>,
    pub warden: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Cast<'info> {
    #[account(mut, seeds=[b"lake", warden.key().as_ref()], bump, has_one=warden)]
    pub lake: Account<'info, LakeCfg>,
    #[account(
        mut,
        constraint = stats.key() != lake.key() @ FfdErr::Dup,
        constraint = stats.key() != card.key() @ FfdErr::Dup
    )]
    pub stats: Account<'info, PoissonStats>,
    #[account(
        mut,
        seeds=[b"card", warden.key().as_ref()], bump,
        constraint = card.key() != trace.key() @ FfdErr::Dup
    )]
    pub card: Account<'info, FishCard>,
    #[account(
        mut,
        constraint = trace.key() != lake.key() @ FfdErr::Dup
    )]
    pub trace: Account<'info, LakeTrace>,
    pub warden: Signer<'info>,
}
#[account] pub struct LakeCfg { pub warden: Pubkey, pub cap: u32, pub state: LakeState }
#[account] pub struct PoissonStats { pub lam_q16: u32, pub count_q16: u32, pub var_q16: u32 }
#[account] pub struct FishCard { pub casts: u32, pub trophies: u32 }
#[account] pub struct LakeTrace { pub notes: u64, pub flags: u32 }
#[error_code] pub enum FfdErr { #[msg("dup")] Dup }