use anchor_lang::prelude::*;

declare_id!("ArtDelV6666666666666666666666666666666666");

#[program]
pub mod art_delete_vuln {
    pub fn delete_art(ctx: Context<DeleteArt>) -> Result<()> {
        // art.owner の検証なし
        let art = &mut ctx.accounts.art;
        art.deleted = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DeleteArt<'info> {
    #[account(mut)]
    pub art: Account<'info, ArtData>,
}

#[account]
pub struct ArtData {
    pub owner: Pubkey,
    pub deleted: bool,
}
