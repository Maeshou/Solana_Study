use anchor_lang::prelude::*;
use anchor_lang::CpiContext;

declare_id!("Ex02CtxFacTory11111111111111111111111111");

#[program]
pub mod context_factory {
    use super::*;

    #[derive(Clone)]
    pub struct PostAccs<'info> {
        pub board: AccountInfo<'info>,
        pub author: AccountInfo<'info>,
    }

    pub fn make_ctx(ctx: Context<MakeCtx>, tag: u64) -> Result<()> {
        // 呼び先プログラム AccountInfo を外部から可変に
        let mut program_ai = ctx.accounts.router_program.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            program_ai = ctx.remaining_accounts[0].clone();
        }

        // CpiContext を“用意”するだけ（ここでは invoke しない）
        let _planned = CpiContext::new(
            program_ai,
            PostAccs {
                board: ctx.accounts.out_board.to_account_info(),
                author: ctx.accounts.user.to_account_info(),
            },
        );

        // 何か状態だけ触って終了
        let st = &mut ctx.accounts.ctx_state;
        st.counter = st.counter.wrapping_add(tag);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MakeCtx<'info> {
    #[account(mut)]
    pub ctx_state: Account<'info, CtxState>,
    /// CHECK:
    pub out_board: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub router_program: AccountInfo<'info>,
}

#[account]
pub struct CtxState { pub counter: u64 }
