use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{Instruction, AccountMeta};

declare_id!("Ex01Bui1dOn1y111111111111111111111111111");

#[program]
pub mod builder_returns_instruction {
    use super::*;

    pub fn plan_dynamic(ctx: Context<Plan>, n: u64) -> Result<()> {
        // 実行先を AccountInfo から決定（差し替え余地あり）
        let mut prg = ctx.accounts.hint_program.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            prg = ctx.remaining_accounts[0].clone();
        }

        // いまは invoke しない：Instruction を作って state にメモるだけ
        let ix = Instruction {
            program_id: *prg.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.actor.key(), false),
            ],
            data: n.to_le_bytes().to_vec(),
        };

        // “後段が invoke 可能な情報” を保存（ここではダイジェストのみ）
        let st = &mut ctx.accounts.plan_state;
        st.program = ix.program_id;
        st.slot0 = ctx.accounts.board.key();
        st.slot1 = ctx.accounts.actor.key();
        st.len = ix.data.len() as u32;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Plan<'info> {
    #[account(mut)]
    pub plan_state: Account<'info, PlanState>,
    /// CHECK:
    pub board: AccountInfo<'info>,
    /// CHECK:
    pub actor: AccountInfo<'info>,
    /// CHECK:
    pub hint_program: AccountInfo<'info>,
}

#[account]
pub struct PlanState {
    pub program: Pubkey,
    pub slot0: Pubkey,
    pub slot1: Pubkey,
    pub len: u32,
}
