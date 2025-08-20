use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("SaFeFixLogCPI111111111111111111111111111");
const LOG_PROGRAM_ID: Pubkey = pubkey!("LoGProg000000000000000000000000000000000");

#[program]
pub mod safe_fixed_log {
    use super::*;

    pub fn write_note(ctx: Context<WriteNote>, value: u64) -> Result<()> {
        let metas = vec![
            AccountMeta::new(ctx.accounts.log_cell.key(), false),
            AccountMeta::new_readonly(ctx.accounts.user.key(), false),
        ];
        let data = value.to_le_bytes().to_vec();

        // 呼び先は定数 LOG_PROGRAM_ID で固定。外部差し替え不可。
        let ix = Instruction { program_id: LOG_PROGRAM_ID, accounts: metas, data };
        invoke(
            &ix,
            &[
                ctx.accounts.log_hint.to_account_info(),
                ctx.accounts.log_cell.to_account_info(),
                ctx.accounts.user.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WriteNote<'info> {
    /// CHECK:
    pub log_hint: AccountInfo<'info>,
    /// CHECK:
    pub log_cell: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
}
