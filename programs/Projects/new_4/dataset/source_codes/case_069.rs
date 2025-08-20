use anchor_lang::prelude::*;

declare_id!("MixInitMissLoop444444444444444444444444444");

#[program]
pub mod example4 {
    use super::*;

    // チャットルームを作成（room にだけ init）
    pub fn create_room(ctx: Context<CreateRoom>, topic: String) -> Result<()> {
        let room = &mut ctx.accounts.room;
        room.topic = topic;
        room.count = 0;
        Ok(())
    }

    // 複数メッセージを投稿（message は init なし）
    pub fn post_messages(ctx: Context<PostMessages>, texts: Vec<String>) -> Result<()> {
        let room = &mut ctx.accounts.room;
        let message = &mut ctx.accounts.message;

        // for ループ内でネスト if
        for txt in texts.iter() {
            if txt.len() > 0 {
                room.count += 1;
                if txt.starts_with("!cmd") {
                    // コマンド扱い（処理略）
                }
            }
        }

        // 最終メッセージをセット
        if !texts.is_empty() {
            message.content = texts[texts.len() - 1].clone();
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRoom<'info> {
    #[account(init, payer = admin, space = 8 + 64 + 4)]
    pub room: Account<'info, Room>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PostMessages<'info> {
    #[account(mut)]
    pub room: Account<'info, Room>,           // ← init なし：既存参照のみ
    pub message: Account<'info, Message>,     // ← init なし（本来は初期化すべき）
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Room {
    pub topic: String,
    pub count: u32,
}

#[account]
pub struct Message {
    pub content: String,
}
