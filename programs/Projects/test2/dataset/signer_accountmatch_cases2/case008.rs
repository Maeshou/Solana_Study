
use anchor_lang::prelude::*;

declare_id!("DocuSys88888888888888888888888888888888888");

#[program]
pub mod case8 {
    use super::*;

    pub fn append_note(ctx: Context<AppendNote>, note: String) -> Result<()> {
        let doc = &mut ctx.accounts.document;
        doc.notes.push(note.clone());
        msg!("Note added: {}", note);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AppendNote<'info> {
    #[account(mut)]
    pub document: Account<'info, Document>,
    /// CHECK: signer and account check are missing
    pub user: UncheckedAccount<'info>,
}

#[account]
pub struct Document {
    pub notes: Vec<String>,
    pub author: Pubkey,
}
