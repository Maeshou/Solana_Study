use anchor_lang::prelude::*;

declare_id!("OwnChkD8000000000000000000000000000000009");

#[program]
pub mod governance_param {
    pub fn set_param(
        ctx: Context<SetParam>,
        key: String,
        value: u64,
    ) -> Result<()> {
        let gov = &mut ctx.accounts.gov;
        // 属性レベルで chair を検証
        gov.params.insert(key.clone(), value);
        gov.change_count = gov.change_count.saturating_add(1);

        // event_log は unchecked
        ctx.accounts.event_log.data.borrow_mut().extend_from_slice(key.as_bytes());
        ctx.accounts.event_log.data.borrow_mut().extend_from_slice(&value.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetParam<'info> {
    #[account(mut, has_one = chair)]
    pub gov: Account<'info, GovernanceData>,
    pub chair: Signer<'info>,
    /// CHECK: イベントログ、所有者検証なし
    #[account(mut)]
    pub event_log: AccountInfo<'info>,
}

#[account]
pub struct GovernanceData {
    pub chair: Pubkey,
    pub params: std::collections::HashMap<String, u64>,
    pub change_count: u64,
}
