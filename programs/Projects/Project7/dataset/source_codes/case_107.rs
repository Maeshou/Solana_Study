// 7) 一定回数までは Token、以降は System を使う
use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer as SysTransfer};
use anchor_spl::token::{self, Transfer as SplTransfer, Token, TokenAccount};

declare_id!("AcctOnlyBurstThenSysFFFFFFFFFFFFFFFFFFFFFF");

#[program]
pub mod burst_then_sys {
    use super::*;
    pub fn init(ctx: Context<InitBurst>, unit_tokens: u64, lamports: u64, burst_len: u64) -> Result<()> {
        let st = &mut ctx.accounts.state;
        st.admin = ctx.accounts.admin.key();
        st.unit_tokens = unit_tokens.max(1);
        st.lamports = lamports;
        st.burst_len = burst_len.max(1);
        st.used = 0;
        Ok(())
    }
    pub fn go(ctx: Context<GoBurst>) -> Result<()> {
        let st = &mut ctx.accounts.state;

        if st.used < st.burst_len {
            token::transfer(CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SplTransfer {
                    from: ctx.accounts.src_tokens.to_account_info(),
                    to: ctx.accounts.dst_tokens.to_account_info(),
                    authority: ctx.accounts.admin.to_account_info(),
                },
            ), st.unit_tokens)?;
            st.used = st.used.saturating_add(1);
        }
        if st.used >= st.burst_len {
            system_program::transfer(CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                SysTransfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.receiver.to_account_info(),
                },
            ), st.lamports)?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBurst<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub state: Account<'info, BurstState>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct GoBurst<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, BurstState>,
    pub admin: Signer<'info>,
    // token
    #[account(mut)] pub src_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub dst_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // system
    #[account(mut)] pub payer: Signer<'info>,
    #[account(mut)] pub receiver: Account<'info, SystemAccount>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct BurstState { pub admin: Pubkey, pub unit_tokens: u64, pub lamports: u64, pub burst_len: u64, pub used: u64 }
