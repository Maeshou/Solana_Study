use anchor_lang::prelude::*;

declare_id!("ChkVar10000000000000000000000000000000001");

#[program]
pub mod task_scheduler {
    pub fn add_task(ctx: Context<AddTask>, desc: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        // has_one + Signer で owner 検証
        assert_keys_eq!(task.owner, ctx.accounts.owner.key());
        task.desc = desc;
        task.timestamp = Clock::get()?.unix_timestamp;

        // audit_log は無検証で RAW データ追記
        let buf = &mut ctx.accounts.audit_log.data.borrow_mut();
        buf.extend_from_slice(b"added;");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddTask<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 256 + 8, has_one = owner)]
    pub task: Account<'info, TaskData>,
    pub owner: Signer<'info>,
    #[account(mut)] pub audit_log: AccountInfo<'info>,  // unchecked
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TaskData {
    pub owner: Pubkey,
    pub desc: String,
    pub timestamp: i64,
}
