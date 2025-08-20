use anchor_lang::prelude::*;

declare_id!("OwnChkCAA0000000000000000000000000000000A");

#[program]
pub mod delegation {
    pub fn delegate(
        ctx: Context<Delegate>,
        to: Pubkey,
    ) -> Result<()> {
        let d = &mut ctx.accounts.delegation;
        // 属性検証で d.voter をチェック
        d.delegated_to = to;
        d.delegate_count = d.delegate_count.saturating_add(1);

        // delegation_log は unchecked
        ctx.accounts.delegation_log.data.borrow_mut().extend_from_slice(&to.to_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Delegate<'info> {
    #[account(mut, has_one = voter)]
    pub delegation: Account<'info, DelegationData>,
    pub voter: Signer<'info>,
    /// CHECK: 委任ログ、所有者検証なし
    #[account(mut)]
    pub delegation_log: AccountInfo<'info>,
}

#[account]
pub struct DelegationData {
    pub voter: Pubkey,
    pub delegated_to: Pubkey,
    pub delegate_count: u64,
}
