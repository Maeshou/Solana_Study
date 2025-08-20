// 1) System と Token をフラグで切替して送金（ラウンドごとに切替）
use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer as SysTransfer};
use anchor_spl::token::{self, Transfer as SplTransfer, Token, TokenAccount};

declare_id!("AcctOnlySwitchSysToken11111111111111111111");

#[program]
pub mod switch_sys_token {
    use super::*;
    pub fn init(ctx: Context<Init>, unit_lamports: u64, unit_tokens: u64, toggle_external: bool) -> Result<()> {
        let st = &mut ctx.accounts.state;
        st.authority = ctx.accounts.authority.key();
        st.unit_lamports = unit_lamports;
        st.unit_tokens = unit_tokens.max(1);
        st.toggle_external = toggle_external;
        Ok(())
    }

    pub fn step(ctx: Context<Step>, rounds: u8) -> Result<()> {
        let st = &mut ctx.accounts.state;
        let mut i: u8 = 0;
        while i < rounds {
            if st.toggle_external {
                // System Program 経由の lamports 転送
                system_program::transfer(CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    SysTransfer {
                        from: ctx.accounts.payer.to_account_info(),
                        to: ctx.accounts.recipient.to_account_info(),
                    },
                ), st.unit_lamports)?;
            }
            if !st.toggle_external {
                // Token Program 経由の SPL 転送
                token::transfer(CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    SplTransfer {
                        from: ctx.accounts.source_tokens.to_account_info(),
                        to: ctx.accounts.dest_tokens.to_account_info(),
                        authority: ctx.accounts.authority.to_account_info(),
                    },
                ), st.unit_tokens)?;
            }
            st.toggle_external = !st.toggle_external;
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8 + 1)]
    pub state: Account<'info, SwitchState>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Step<'info> {
    #[account(mut, has_one = authority)]
    pub state: Account<'info, SwitchState>,
    pub authority: Signer<'info>,

    // system transfer 用
    #[account(mut)] pub payer: Signer<'info>,
    /// 受け取り先（任意のアカウントだが Account<T> で受ける）
    #[account(mut)] pub recipient: Account<'info, SystemAccount>,

    // token transfer 用
    #[account(mut)] pub source_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub dest_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}
#[account]
pub struct SwitchState {
    pub authority: Pubkey,
    pub unit_lamports: u64,
    pub unit_tokens: u64,
    pub toggle_external: bool,
}
