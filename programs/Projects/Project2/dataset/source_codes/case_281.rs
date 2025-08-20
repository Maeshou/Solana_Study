use anchor_lang::prelude::*;

declare_id!("MigrationLog000000000000000000000000000000");

#[program]
pub mod migration_log {
    use super::*;

    pub fn record_migration(ctx: Context<LogMigration>, nft_id: u64) -> Result<()> {
        let ml = &mut ctx.accounts.log;
        ml.entries.push((nft_id, ctx.accounts.user.key()));
        if ml.entries.len() > 50 {
            ml.entries.remove(0);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMigration<'info> {
    #[account(mut)]
    pub log: Account<'info, MigrationData>,
    pub user: Signer<'info>,
}

#[account]
pub struct MigrationData {
    pub entries: Vec<(u64, Pubkey)>,
}
