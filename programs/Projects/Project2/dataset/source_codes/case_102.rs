use anchor_lang::prelude::*;

declare_id!("Count111111111111111111111111111111111111");

#[program]
pub mod counter_program {
    use super::*;

    /// カウンターアカウント(PDA)の初期化
    pub fn init_counter(ctx: Context<InitCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = ctx.accounts.authority.key();
        counter.count     = 0;
        counter.bump      = *ctx.bumps.get("counter").unwrap();
        Ok(())
    }

    /// カウンターを +1 する
    pub fn increment(ctx: Context<ModifyCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter
            .count
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }

    /// カウンターを -1 する
    pub fn decrement(ctx: Context<ModifyCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter
            .count
            .checked_sub(1)
            .ok_or(ErrorCode::Underflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCounter<'info> {
    /// PDA：["counter", authority], bump付き。再初期化を防止
    #[account(
        init,
        seeds  = [b"counter", authority.key().as_ref()],
        bump,
        payer  = authority,
        space  = 8 + 32 + 8 + 1   // discriminator + authority + count + bump
    )]
    pub counter:   Account<'info, CounterAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyCounter<'info> {
    /// has_one＋PDA固定でauthority==counter.authorityを検証
    #[account(
        mut,
        seeds   = [b"counter", authority.key().as_ref()],
        bump    = counter.bump,
        has_one = authority
    )]
    pub counter:   Account<'info, CounterAccount>,

    pub authority: Signer<'info>,
}

#[account]
pub struct CounterAccount {
    /// 操作を許可されたユーザー
    pub authority: Pubkey,
    /// カウンタ値
    pub count:     u64,
    /// PDA用bump
    pub bump:      u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("オーバーフローしました")]
    Overflow,
    #[msg("アンダーフローしました")]
    Underflow,
}
