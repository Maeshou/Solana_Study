use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体として定義 ──
#[account]
#[derive(Default)]
pub struct Lottery(pub Vec<Pubkey>);

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUQ");

#[program]
pub mod lottery {
    use super::*;

    /// Lottery を初期化（PDA のみ生成。Vec は Default::default() で空に）
    pub fn initialize(
        ctx: Context<Initialize>,
    ) -> Result<()> {
        // 何も代入せず、タプル内の Vec が空のデフォルトで始まる
        Ok(())
    }

    /// 参加者登録：自身の Pubkey を participants に追加
    pub fn join(
        ctx: Context<Join>,
    ) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery.0;
        lottery.push(ctx.accounts.user.key());
        Ok(())
    }

    /// 当選者決定：clock のタイムスタンプを乱数源に使い、participants から選出
    pub fn pick_winner(
        ctx: Context<PickWinner>,
    ) -> Result<Pubkey> {
        let participants = &mut ctx.accounts.lottery.0;
        let now = ctx.accounts.clock.unix_timestamp as usize;
        if participants.is_empty() {
            return Err(error!(ErrorCode::NoParticipants));
        }
        let idx = now % participants.len();
        let winner = participants[idx];
        // 当選後はクリア
        participants.clear();
        Ok(winner)
    }

    /// リセット：参加者一覧をクリア
    pub fn reset(
        ctx: Context<Reset>,
    ) -> Result<()> {
        ctx.accounts.lottery.0.clear();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// PDA を init するだけ。内部 Vec はデフォルトで空
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"lottery", authority.key().as_ref()],
        bump,
        space = 8 + (4 + 10 * 32)  // discriminator + Vec<Pubkey> (max 10人)
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Join<'info> {
    #[account(
        mut,
        seeds = [b"lottery", authority.key().as_ref()],
        bump = lottery.to_account_info().data.borrow()[0], // bump 不保持
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PickWinner<'info> {
    #[account(
        mut,
        seeds = [b"lottery", authority.key().as_ref()],
        bump = lottery.to_account_info().data.borrow()[0],
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(
        mut,
        seeds = [b"lottery", authority.key().as_ref()],
        bump = lottery.to_account_info().data.borrow()[0],
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("No participants in the lottery")]
    NoParticipants,
}
