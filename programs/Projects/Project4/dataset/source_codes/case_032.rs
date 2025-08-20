use anchor_lang::prelude::*;

declare_id!("Ex8000000000000000000000000000000000008");

#[program]
pub mod example8 {
    use super::*;

    // チャットルームを作成し、作成時刻を保持
    pub fn create_room(ctx: Context<CreateRoom>, topic: String) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let rm = &mut ctx.accounts.room;            // ← initあり
        rm.topic = topic;
        rm.count = 0;
        rm.created_at = now;
        Ok(())
    }

    // メッセージを投稿し、最新メッセージと投稿数を更新
    pub fn post_message(ctx: Context<PostMessage>, msg: String) -> Result<()> {
        let rm = &mut ctx.accounts.room;            // ← initなし：既存参照のみ
        if msg.len() > 0 {
            rm.count += 1;
            rm.last_message = msg.clone();
            rm.last_posted_at = Clock::get()?.unix_timestamp;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRoom<'info> {
    #[account(init, payer = admin, space = 8 + 64 + 4 + 8*3)]
    pub room: Account<'info, RoomData>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PostMessage<'info> {
    pub room: Account<'info, RoomData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RoomData {
    pub topic: String,
    pub count: u32,
    pub created_at: i64,
    pub last_message: String,
    pub last_posted_at: i64,
}
