use anchor_lang::prelude::*;

declare_id!("VulnInit9999999999999999999999999999999999");

#[program]
pub mod vuln_document {
    use super::*;

    pub fn init_document(
        ctx: Context<InitDocument>,
        cid: String,
    ) -> Result<()> {
        let doc = &mut ctx.accounts.document;        // ← Init OK
        doc.cid   = cid.clone();
        doc.owner = ctx.accounts.author.key();

        let sigs = &mut ctx.accounts.signatures;     // ← Init OK
        sigs.signers = Vec::new();

        let log = &mut ctx.accounts.verification_log; // ← Init missing
        log.entries = Vec::new();
        log.entries.push(format!("Document {} created", cid));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDocument<'info> {
    #[account(init, payer = author, space = 8 + 4 + 200 + 32)]
    pub document: Account<'info, DocumentData>,
    #[account(init, payer = author, space = 8 + 4 + (32 * 10))]
    pub signatures: Account<'info, SignatureListData>,
    pub verification_log: Account<'info, VerificationLogData>, // ← init がない
    #[account(mut)] pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DocumentData {
    pub cid: String,
    pub owner: Pubkey,
}

#[account]
pub struct SignatureListData {
    pub signers: Vec<Pubkey>,
}

#[account]
pub struct VerificationLogData {
    pub entries: Vec<String>,
}
