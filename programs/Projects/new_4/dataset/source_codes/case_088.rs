use anchor_lang::prelude::*;

declare_id!("Repertory17Doc1111111111111111111111111111111");

#[program]
pub mod docsig {
    use super::*;

    // ドキュメントを登録
    pub fn register_doc(ctx: Context<RegisterDoc>, cid: String) -> Result<()> {
        let d = &mut ctx.accounts.document;
        d.cid = cid;
        d.signatures = 0;
        Ok(())
    }

    // 署名を追加
    pub fn sign_doc(ctx: Context<SignDoc>, pubkeys: Vec<Pubkey>) -> Result<()> {
        let d = &mut ctx.accounts.document;       // ← initなし：既存参照
        for pk in pubkeys.iter() {
            if *pk != Pubkey::default() {
                d.signatures += 1;
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterDoc<'info> {
    #[account(init, payer = user, space = 8 + 4 + 200 + 4)]
    pub document: Account<'info, DocumentData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignDoc<'info> {
    pub document: Account<'info, DocumentData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DocumentData {
    pub cid: String,
    pub signatures: u32,
}
