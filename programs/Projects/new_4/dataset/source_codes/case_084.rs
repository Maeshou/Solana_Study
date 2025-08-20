use anchor_lang::prelude::*;

declare_id!("Repertory13Nft1111111111111111111111111111111");

#[program]
pub mod nft_mint {
    use super::*;

    // NFTミントアカウントを作成
    pub fn init_mint(ctx: Context<InitMint>, uri: String) -> Result<()> {
        let m = &mut ctx.accounts.mint;
        m.minter = ctx.accounts.minter.key();
        m.uri = uri;
        m.supply = 1;
        Ok(())
    }

    // メタデータを追記
    pub fn set_metadata(ctx: Context<SetMetadata>, name: String) -> Result<()> {
        let meta = &mut ctx.accounts.metadata; // ← initなし：既存参照
        meta.name = name;
        meta.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMint<'info> {
    #[account(init, payer = minter, space = 8 + 32 + 4 + 200 + 8)]
    pub mint: Account<'info, MintData>,
    #[account(mut)] pub minter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetMetadata<'info> {
    pub metadata: Account<'info, Metadata>,
    #[account(mut)] pub minter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MintData {
    pub minter: Pubkey,
    pub uri: String,
    pub supply: u8,
}

#[account]
pub struct Metadata {
    pub name: String,
    pub created_at: i64,
}
