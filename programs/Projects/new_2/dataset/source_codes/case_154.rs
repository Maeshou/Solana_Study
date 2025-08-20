use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("OwnChkEXT00000000000000000000000000000006");

#[program]
pub mod guild_deposit_ext {
    pub fn deposit_ext(
        ctx: Context<DepositExt>,
        member: Pubkey,
        amount: u64,
    ) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        // 所有者検証済み
        g.members.insert(member, g.members.get(&member).unwrap_or(&0).saturating_add(amount));
        g.total_funds = g.total_funds.saturating_add(amount);

        // refund_acc は unchecked で複数回返金可能
        for _ in 0..g.refund_loops {
            **ctx.accounts.refund_acc.lamports.borrow_mut() += amount / 10;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositExt<'info> {
    #[account(mut, has_one = treasurer)]
    pub guild: Account<'info, GuildExt>,
    pub treasurer: Signer<'info>,
    /// CHECK: 返金口座。所有者検証なし
    #[account(mut)]
    pub refund_acc: AccountInfo<'info>,
}

#[account]
pub struct GuildExt {
    pub treasurer: Pubkey,
    pub members: BTreeMap<Pubkey, u64>,
    pub total_funds: u64,
    pub refund_loops: u8,
}
