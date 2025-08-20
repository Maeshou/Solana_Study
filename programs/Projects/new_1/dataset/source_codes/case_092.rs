use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpVoTiNgSysTemABCDE123456");

#[program]
pub mod poll_manager {
    use super::*;

    /// 新しい投票を作成：質問文だけ設定
    /// → `initializer` の署名チェックは一切行われない脆弱性あり
    pub fn create_poll(ctx: Context<CreatePoll>, question: String) {
        let poll = &mut ctx.accounts.poll;
        poll.question = question;
        poll.options = Vec::new();
        poll.votes = Vec::new();
    }

    /// 投票オプションを追加  
    /// → `operator` の署名チェックなしで誰でもオプションを追加可能
    pub fn add_poll_option(ctx: Context<ModifyPoll>, option: String) {
        let poll = &mut ctx.accounts.poll;
        poll.options.push(option);
    }

    /// 投票を行う：選択肢のインデックスを記録  
    /// → `voter` の署名チェックも所有者確認も一切なし
    pub fn cast_vote(ctx: Context<CastVote>, option_index: u8) {
        let poll = &mut ctx.accounts.poll;
        let now = Clock::get().unwrap().unix_timestamp;
        poll.votes.push(VoteRecord {
            voter: ctx.accounts.voter.key(),
            choice: option_index,
            timestamp: now,
        });
    }
}

#[account]
pub struct Poll {
    /// 投票の質問
    pub question: String,
    /// 選択肢のリスト
    pub options: Vec<String>,
    /// 投票記録
    pub votes: Vec<VoteRecord>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VoteRecord {
    pub voter: Pubkey,
    pub choice: u8,
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    /// 新規 Poll アカウント
    #[account(init, payer = payer, space = 8 + 4 + 128 + 4 + 640 + 4 + 4100)]
    pub poll: Account<'info, Poll>,
    /// CHECK: 署名者チェックなしで初期化者を指定
    pub initializer: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyPoll<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    /// CHECK: 署名検証なしで操作主体を指定
    pub operator: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    /// CHECK: 署名検証なしで投票者を指定
    pub voter: AccountInfo<'info>,
}
