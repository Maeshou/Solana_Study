use anchor_lang::prelude::*;

declare_id!("OwnChkE900000000000000000000000000000000A");

#[program]
pub mod event_manager {
    pub fn register_event(
        ctx: Context<RegisterEvent>,
        name: String,
        timestamp: u64,
    ) -> Result<()> {
        let em = &mut ctx.accounts.manager;
        // 属性レベルで manager を検証
        em.events.push((name.clone(), timestamp));
        em.reg_count = em.reg_count.saturating_add(1);

        // backup_log は unchecked
        let mut log = ctx.accounts.backup_log.data.borrow_mut();
        log.extend_from_slice(name.as_bytes());
        log.extend_from_slice(&timestamp.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterEvent<'info> {
    #[account(mut, has_one = manager)]
    pub manager: Account<'info, EventManager>,
    pub manager_signer: Signer<'info>,
    /// CHECK: バックアップログ、所有者検証なし
    #[account(mut)]
    pub backup_log: AccountInfo<'info>,
}

#[account]
pub struct EventManager {
    pub manager: Pubkey,
    pub events: Vec<(String, u64)>,
    pub reg_count: u64,
}
