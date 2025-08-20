// 6. ドキュメント署名＋署名ログ
use anchor_lang::prelude::*;
declare_id!("DOCS111122223333444455556666777788");

#[program]
pub mod misinit_docs_v7 {
    use super::*;

    pub fn init_document(
        ctx: Context<InitDoc>,
        title: String,
    ) -> Result<()> {
        let d = &mut ctx.accounts.document;
        d.title = title;
        d.signed = false;
        Ok(())
    }

    pub fn sign_document(ctx: Context<InitDoc>) -> Result<()> {
        let d = &mut ctx.accounts.document;
        d.signed = true;
        Ok(())
    }

    pub fn log_signature(
        ctx: Context<InitDoc>,
        signer: Pubkey,
    ) -> Result<()> {
        let log = &mut ctx.accounts.sign_log;
        log.signers.push(signer);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDoc<'info> {
    #[account(init, payer = author, space = 8 + (4+64) + 1)] pub document: Account<'info, DocumentData>,
    #[account(mut)] pub sign_log: Account<'info, SignLog>,
    #[account(mut)] pub author: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct DocumentData { pub title: String, pub signed: bool }
#[account]
pub struct SignLog { pub signers: Vec<Pubkey> }
