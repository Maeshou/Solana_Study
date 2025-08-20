use anchor_lang::prelude::*;

declare_id!("VulnVarX9000000000000000000000000000000009");

#[program]
pub mod example9 {
    pub fn archive_data(ctx: Context<Ctx9>) -> Result<()> {
        // staging_acc は unchecked
        let staging = ctx.accounts.staging_acc.data.borrow();
        // archive は has_one 検証済み
        let arc = &mut ctx.accounts.archive;
        arc.records.push(staging.to_vec());
        arc.archive_count = arc.archive_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx9<'info> {
    /// CHECK: ステージングアカウント、所有者検証なし
    #[account(mut)]
    pub staging_acc: AccountInfo<'info>,

    #[account(mut, has_one = owner)]
    pub archive: Account<'info, ArchiveData>,
    pub owner: Signer<'info>,
}

#[account]
pub struct ArchiveData {
    pub owner: Pubkey,
    pub records: Vec<Vec<u8>>,
    pub archive_count: u64,
}
