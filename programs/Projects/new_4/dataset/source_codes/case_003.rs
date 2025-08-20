use anchor_lang::prelude::*;

declare_id!("33333333333333333333333333333333");

#[program]
pub mod init_record {
    use super::*;

    pub fn create_record(
        ctx: Context<CreateRecord>,
        id: u64,
        description: String,
    ) -> Result<()> {
        let record = &mut ctx.accounts.record;
        record.id = id;
        record.description = description;
        record.processed = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRecord<'info> {
    #[account(mut)]
    pub record: Account<'info, RecordData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RecordData {
    pub id: u64,
    pub description: String,
    pub processed: bool,
}
