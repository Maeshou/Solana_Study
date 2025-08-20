use anchor_lang::prelude::*;

declare_id!("Ex03DefQueuE111111111111111111111111111");

#[program]
pub mod deferred_queue {
    use super::*;

    pub fn enqueue(ctx: Context<Enqueue>, amount: u64) -> Result<()> {
        // 呼び先の鍵を外部 AccountInfo から
        let mut prg = ctx.accounts.exec_program.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            prg = ctx.remaining_accounts[0].clone();
        }

        // 実行計画（超簡素版）を口座に積む：いまは invoke しない
        let plan = PlanItem {
            program: *prg.key,
            a0: ctx.accounts.slot_a.key(),
            a1: ctx.accounts.slot_b.key(),
            val: amount,
        };
        let q = &mut ctx.accounts.queue;
        q.items.push(plan);
        q.total = q.total.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Enqueue<'info> {
    #[account(mut)]
    pub queue: Account<'info, ExecQueue>,
    /// CHECK:
    pub slot_a: AccountInfo<'info>,
    /// CHECK:
    pub slot_b: AccountInfo<'info>,
    /// CHECK:
    pub exec_program: AccountInfo<'info>,
}

#[account]
pub struct ExecQueue {
    pub items: Vec<PlanItem>,
    pub total: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PlanItem {
    pub program: Pubkey,
    pub a0: Pubkey,
    pub a1: Pubkey,
    pub val: u64,
}
