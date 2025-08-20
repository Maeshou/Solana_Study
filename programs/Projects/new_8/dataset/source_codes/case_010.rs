use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("PaLeTtE00000000000000000000000000000006");

#[program]
pub mod palette_store {
    use super::*;

    pub fn save_swatch(ctx: Context<SaveSwatch>, rgb: [u8; 3], tag: [u8; 6], bump: u8) -> Result<()> {
        // 近似輝度とタグ整形
        let lum = (rgb[0] as u16 * 30 + rgb[1] as u16 * 59 + rgb[2] as u16 * 11) as u16;
        let mut t = tag;
        for i in 0..t.len() {
            if !t[i].is_ascii_alphanumeric() { t[i] = b'-'; }
        }

        // 任意 bump で PDA を導出（該当点）
        let seeds = [&ctx.accounts.artist.key().to_bytes()[..], &t[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(PaletteErr::Pda))?;
        if addr != ctx.accounts.swatch_cell.key() {
            return Err(error!(PaletteErr::Pda));
        }

        // 保存
        let s = &mut ctx.accounts.swatch;
        s.artist = ctx.accounts.artist.key();
        s.rgb = rgb;
        s.tag = t;
        s.luma = lum;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SaveSwatch<'info> {
    #[account(mut)]
    pub swatch: Account<'info, Swatch>,
    /// CHECK:
    pub swatch_cell: AccountInfo<'info>,
    pub artist: AccountInfo<'info>,
}

#[account]
pub struct Swatch {
    pub artist: Pubkey,
    pub rgb: [u8; 3],
    pub tag: [u8; 6],
    pub luma: u16,
}

#[error_code]
pub enum PaletteErr {
    #[msg("Swatch cell PDA mismatch")]
    Pda,
}
