use anchor_lang::prelude::*;

// ============================================================================
// 1) Sigil Museum — 展示スコア（PDAなし / has_oneで博物館ひも付け）
// ============================================================================
declare_id!("SGMS11111111111111111111111111111111111111111");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ExhibitState { Draft, Open, Seal }

#[program]
pub mod sigil_museum {
    use super::*;
    use ExhibitState::*;

    pub fn init_museum(ctx: Context<InitMuseum>, cap: u32) -> Result<()> {
        let a = &mut ctx.accounts;
        a.museum.curator = a.curator.key();
        a.museum.cap = cap;
        a.museum.state = Draft;

        a.relic_a.museum = a.museum.key();
        a.relic_b.museum = a.museum.key();
        a.log.museum = a.museum.key();
        Ok(())
    }

    pub fn update_exhibit(ctx: Context<UpdateExhibit>, rounds: u32) -> Result<()> {
        let a = &mut ctx.accounts;

        // ループ：交互に重みを変える
        for i in 0..rounds {
            let w = 3 + (i % 5);
            a.relic_a.score = a.relic_a.score.wrapping_add(w * 7);
            a.relic_b.score = a.relic_b.score.wrapping_add((w + 2) * 5);
            a.log.events = a.log.events.wrapping_add(1);
        }

        let sum = a.relic_a.score as u64 + a.relic_b.score as u64;
        if sum > a.museum.cap as u64 {
            a.museum.state = Seal;
            a.log.flags = a.log.flags.wrapping_add(2);
            a.relic_a.score = a.relic_a.score / 2 + 11;
            a.relic_b.score = a.relic_b.score / 2 + 13;
            msg!("sealed: flags+2, both scores halved+adj");
        } else {
            a.museum.state = Open;
            a.log.checks = a.log.checks.wrapping_add(1);
            a.relic_a.score = a.relic_a.score.rotate_left(1);
            a.relic_b.score = a.relic_b.score.rotate_right(1);
            msg!("open: checks+1, rotate scores");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMuseum<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub museum: Account<'info, Museum>,
    #[account(init, payer=payer, space=8+32+4)]
    pub relic_a: Account<'info, Relic>,
    #[account(init, payer=payer, space=8+32+4)]
    pub relic_b: Account<'info, Relic>,
    #[account(init, payer=payer, space=8+32+8+4)]
    pub log: Account<'info, ExhibitLog>,
    #[account(mut)] pub payer: Signer<'info>,
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateExhibit<'info> {
    #[account(mut, has_one=curator)]
    pub museum: Account<'info, Museum>,
    #[account(
        mut,
        has_one=museum,
        constraint = relic_a.key() != relic_b.key() @ MsErr::Dup,
        constraint = relic_a.key() != log.key() @ MsErr::Dup
    )]
    pub relic_a: Account<'info, Relic>,
    #[account(mut, has_one=museum)]
    pub relic_b: Account<'info, Relic>,
    #[account(mut, has_one=museum)]
    pub log: Account<'info, ExhibitLog>,
    pub curator: Signer<'info>,
}

#[account] pub struct Museum { pub curator: Pubkey, pub cap: u32, pub state: ExhibitState }
#[account] pub struct Relic { pub museum: Pubkey, pub score: u32 }
#[account] pub struct ExhibitLog { pub museum: Pubkey, pub events: u64, pub checks: u32, pub flags: u32 }
#[error_code] pub enum MsErr { #[msg("duplicate mutable account")] Dup }
