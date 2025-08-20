use anchor_lang::prelude::*;

declare_id!("ArtCanvas1111111111111111111111111111111111");

#[program]
pub mod art_canvas {
    use super::*;

    pub fn submit_artwork(ctx: Context<SubmitArt>, palette: [u8; 3]) -> Result<()> {
        let artist = &ctx.accounts.entity_one;
        let gallery = &ctx.accounts.entity_two;
        let canvas = &mut ctx.accounts.art_canvas;

        for i in 0..3 {
            canvas.data.borrow_mut()[i] = palette[i];
        }

        canvas.data.borrow_mut()[3] = artist.key.as_ref()[0];
        canvas.data.borrow_mut()[4] = gallery.key.as_ref()[0];

        if artist.key == gallery.key {
            canvas.data.borrow_mut()[5] = 0xFF;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitArt<'info> {
    #[account(mut)]
    pub entity_one: AccountInfo<'info>, // Could be artist
    #[account(mut)]
    pub entity_two: AccountInfo<'info>, // Could be gallery
    #[account(mut)]
    pub art_canvas: AccountInfo<'info>,
}
