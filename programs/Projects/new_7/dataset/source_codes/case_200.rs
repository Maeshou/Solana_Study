use anchor_lang::prelude::*;

declare_id!("Ex04LogOn1y1111111111111111111111111111");

#[program]
pub mod log_only_planner {
    use super::*;

    pub fn draft(ctx: Context<Draft>, z: u64) -> Result<()> {
        let mut target = ctx.accounts.target_program.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            target = ctx.remaining_accounts[0].clone();
        }

        // 実行しない：ログと状態更新のみ
        msg!("planned.program = {}", target.key());
        msg!("planned.args_len = {}", z.to_le_bytes().len());

        let s = &mut ctx.accounts.draft_state;
        s.last_prog = *target.key();
        s.param = z.rotate_left(3);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Draft<'info> {
    #[account(mut)]
    pub draft_state: Account<'info, DraftState>,
    /// CHECK:
    pub target_program: AccountInfo<'info>,
}

#[account]
pub struct DraftState {
    pub last_prog: Pubkey,
    pub param: u64,
}
