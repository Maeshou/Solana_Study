use anchor_lang::prelude::*;

declare_id!("ArtModHist0088888888888888888888888888888888");

#[program]
pub mod art_modify {
    use super::*;

    pub fn modify_art(ctx: Context<ModifyArt>, nft_id: u64, desc: String) -> Result<()> {
        let hist = &mut ctx.accounts.history;
        hist.log
            .entry(nft_id)
            .and_modify(|v| {
                v[hist.index as usize] = desc.clone();
            })
            .or_insert_with(|| vec![desc.clone(); hist.capacity as usize]);
        hist.index = (hist.index + 1) % hist.capacity;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyArt<'info> {
    #[account(mut)]
    pub history: Account<'info, ArtHistory>,
}

#[account]
pub struct ArtHistory {
    pub log: std::collections::BTreeMap<u64, Vec<String>>,
    pub capacity: u8,
    pub index: u8,
}
