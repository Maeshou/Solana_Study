use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzT7");

#[program]
pub mod note_manager {
    use super::*;

    /// ノート初期化：PDA で生成し、所有者と内容を保存
    pub fn initialize_note(
        ctx: Context<InitializeNote>,
        bump: u8,
        content: String,
    ) -> Result<()> {
        let note = &mut ctx.accounts.note;
        note.owner = ctx.accounts.user.key();
        note.bump = bump;
        note.content = content;
        Ok(())
    }

    /// ノート更新：has_one と signer で所有者を検証
    pub fn update_content(
        ctx: Context<UpdateContent>,
        new_content: String,
    ) -> Result<()> {
        let note = &mut ctx.accounts.note;
        note.content = new_content;
        Ok(())
    }
}

/// 初期化用アカウント定義
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeNote<'info> {
    /// PDA で生成するノートアカウント
    #[account(
        init,
        payer = user,
        // space = discriminator(8) + owner Pubkey(32) + bump(1) + String len prefix(4) + 最大256バイトの content
        space = 8 + 32 + 1 + 4 + 256,
        seeds = [b"note", user.key().as_ref()],
        bump
    )]
    pub note: Account<'info, Note>,

    /// トランザクション送信者（オーナー）
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// 更新用アカウント定義
#[derive(Accounts)]
pub struct UpdateContent<'info> {
    /// 既存のノート（PDA seeds＋bump、has_one=owner）
    #[account(
        mut,
        seeds = [b"note", owner.key().as_ref()],
        bump = note.bump,
        has_one = owner
    )]
    pub note: Account<'info, Note>,

    /// ノート所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

/// Note データ構造：所有者、bump、内容を保持
#[account]
pub struct Note {
    pub owner: Pubkey,
    pub bump: u8,
    pub content: String,
}

/// カスタムエラー（今回は属性チェックのみで不変条件を担保するため空）
#[error_code]
pub enum ErrorCode {}
