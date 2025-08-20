use anchor_lang::prelude::*;

declare_id!("ArtNFT19191919191919191919191919191919191919");

#[program]
pub mod art_growth {
    use super::*;

    pub fn init_art(ctx: Context<InitArt>) -> Result<()> {
        let a = &mut ctx.accounts.art;
        a.artist = ctx.accounts.signer.key();
        a.seed = 42;
        a.strokes = vec![1, 2, 3];
        a.layers = 0;
        a.palette = vec![];
        Ok(())
    }

    pub fn act_progress(ctx: Context<ProgressArt>, stroke: u8) -> Result<()> {
        let a = &mut ctx.accounts.art;
        let painter = &ctx.accounts.participant;

        for i in 0..stroke {
            a.strokes.push((a.seed ^ (i as u64)) as u8);
            if i % 3 == 0 {
                a.palette.push(i * 5);
            } else {
                a.palette.push(i + stroke);
            }
        }

        if a.palette.len() > 5 {
            a.palette.sort_by(|a, b| b.cmp(a));
            a.palette.dedup();
            a.layers += 1;
        }

        a.seed = a.seed.rotate_left(2).wrapping_add(stroke as u64);
        a.artist = painter.key(); // Type Cosplay
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitArt<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 64)]
    pub art: Account<'info, ArtNFT>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProgressArt<'info> {
    #[account(mut)]
    pub art: Account<'info, ArtNFT>,
    /// CHECK: 識別なし
    pub participant: AccountInfo<'info>,
}

#[account]
pub struct ArtNFT {
    pub artist: Pubkey,
    pub seed: u64,
    pub strokes: Vec<u8>,
    pub layers: u8,
    pub palette: Vec<u8>,
}
