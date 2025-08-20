use anchor_lang::prelude::*;

declare_id!("Poll111111111111111111111111111111111111");

#[program]
pub mod poll_manager {
    /// 新規 Poll（アンケート）作成
    pub fn create_poll(
        ctx: Context<CreatePoll>,
        title: String,
        description: String,
    ) -> Result<()> {
        // 入力長チェック（オーバーフロー／メモリ攻撃防止）
        require!(title.len() <= 64, ErrorCode::TitleTooLong);
        require!(description.len() <= 256, ErrorCode::DescriptionTooLong);

        let poll = &mut ctx.accounts.poll;
        // Signer Authorization ＋ Owner Check
        poll.owner       = ctx.accounts.user.key();
        poll.title       = title;
        poll.description = description;
        Ok(())
    }

    /// Poll のタイトル／説明を更新（作成者のみ）
    pub fn update_poll(
        ctx: Context<UpdatePoll>,
        new_title: String,
        new_description: String,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        // Account Matching + Signer Authorization
        require!(
            poll.owner == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );
        // 入力長チェック
        require!(new_title.len() <= 64,  ErrorCode::TitleTooLong);
        require!(new_description.len() <= 256, ErrorCode::DescriptionTooLong);

        poll.title       = new_title;
        poll.description = new_description;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    /// init 制約で同一アカウント再初期化を防止（Reinit Attack）
    #[account(init, payer = user, space = 8 + 32 + 4 + 64 + 4 + 256)]
    pub poll:          Account<'info, PollAccount>,

    /// Poll 作成者（署名者）
    #[account(mut)]
    pub user:          Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePoll<'info> {
    /// Anchor の Account<> による Owner Check / Type Cosplay
    #[account(mut)]
    pub poll:          Account<'info, PollAccount>,

    /// 実際に署名したユーザー（Signer Authorization）
    pub user:          Signer<'info>,
}

#[account]
pub struct PollAccount {
    /// この Poll を操作できるユーザー
    pub owner:        Pubkey,
    /// タイトル（最大64文字）
    pub title:        String,
    /// 説明（最大256文字）
    pub description:  String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Title is too long")]
    TitleTooLong,
    #[msg("Description is too long")]
    DescriptionTooLong,
}
