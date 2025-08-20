use anchor_lang::prelude::*;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpD1E2F3G4H5J6K7L8M9N0O1P2Q3R4");

#[program]
pub mod ping {
    use super::*;

    /// 呼び出されるたびにカウンターをインクリメント
    pub fn ping(ctx: Context<Ping>) -> ProgramResult {
        let state = &mut ctx.accounts.state;
        state.counter = state.counter.wrapping_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Ping<'info> {
    #[account(
        mut,
        seeds = [b"state", owner.key().as_ref()],
        bump = bump,
        has_one = owner,
    )]
    pub state: Account<'info, State>,

    /// アカウント操作の署名者
    pub owner: Signer<'info>,
}

#[account]
pub struct State {
    /// カウンターの所有者
    pub owner: Pubkey,
    /// PDA生成用バンプ
    pub bump: u8,
    /// 呼び出し回数を保存
    pub counter: u64,
}

// エラー定義は不要（分岐なし）
