
// ============================================================================
// B系: ハッシュ主体（決定論的; Anchor属性のみでガード）
// ============================================================================

// 4) Sigil Weave Hash — 色相合成（PDAあり）
declare_id!("SWHH444444444444444444444444444444444");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LoomState { Draft, Flow, Seal }

#[program]
pub mod sigil_weave_hash {
    use super::*;
    use LoomState::*;

    pub fn init_loom(ctx: Context<InitLoomHash>, limit: u32) -> Result<()> {
        let a = &mut ctx.accounts;
        a.loom.artist = a.artist.key();
        a.loom.limit = limit;
        a.loom.state = Draft;
        Ok(())
    }

    pub fn weave(ctx: Context<DoWeave>, passes: u32) -> Result<()> {
        let a = &mut ctx.accounts;

        for i in 0..passes {
            let seed = [&a.loom.artist.as_ref(), &a.palette.hues.to_le_bytes(), &i.to_le_bytes()];
            let h = hashv(&seed);
            let bump = u16::from_le_bytes([h.0[0], h.0[1]]) as u32 % 97 + 3;

            a.thread_a.strands = a.thread_a.strands.wrapping_add(bump);
            a.thread_b.strands = a.thread_b.strands.wrapping_add(bump ^ 0x55);
            a.palette.hues = a.palette.hues.rotate_left((bump % 13) + 1);
            a.log.mixes = a.log.mixes.wrapping_add(1);
        }

        let sum = a.thread_a.strands.wrapping_add(a.thread_b.strands);
        if sum > a.loom.limit {
            a.loom.state = Seal;
            a.log.events = a.log.events.wrapping_add(2);
            a.palette.hues ^= 0xA5A5_5A5A;
            a.thread_b.strands = a.thread_b.strands ^ (a.thread_a.strands >> 1);
            msg!("sealed: events+2, palette xor, cross mask");
        } else {
            a.loom.state = Flow;
            a.thread_a.strands = a.thread_a.strands.rotate_left(2);
            a.thread_b.strands = a.thread_b.strands.rotate_right(3);
            a.log.mixes = a.log.mixes.wrapping_add(1);
            msg!("flow: rotate strands, mixes+1");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLoomHash<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub loom: Account<'info, HashLoom>,
    #[account(init, payer=payer, space=8+4)]
    pub thread_a: Account<'info, Thread>,
    #[account(init, payer=payer, space=8+4)]
    pub thread_b: Account<'info, Thread>,
    #[account(init, payer=payer, space=8+4, seeds=[b"palette", artist.key().as_ref()], bump)]
    pub palette: Account<'info, PaletteHash>,
    #[account(init, payer=payer, space=8+4+4)]
    pub log: Account<'info, MixLog>,
    #[account(mut)] pub payer: Signer<'info>,
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DoWeave<'info> {
    #[account(mut)]
    pub loom: Account<'info, HashLoom>,
    #[account(
        mut,
        constraint = thread_a.key() != thread_b.key() @ SwErr::Dup,
        constraint = thread_a.key() != palette.key() @ SwErr::Dup
    )]
    pub thread_a: Account<'info, Thread>,
    #[account(
        mut,
        constraint = thread_b.key() != palette.key() @ SwErr::Dup
    )]
    pub thread_b: Account<'info, Thread>,
    #[account(mut, seeds=[b"palette", artist.key().as_ref()], bump)]
    pub palette: Account<'info, PaletteHash>,
    #[account(
        mut,
        constraint = log.key() != palette.key() @ SwErr::Dup
    )]
    pub log: Account<'info, MixLog>,
    pub artist: Signer<'info>,
}

#[account] pub struct HashLoom { pub artist: Pubkey, pub limit: u32, pub state: LoomState }
#[account] pub struct Thread { pub strands: u32 }
#[account] pub struct PaletteHash { pub hues: u32 }
#[account] pub struct MixLog { pub mixes: u32, pub events: u32 }
#[error_code] pub enum SwErr { #[msg("dup")] Dup }
