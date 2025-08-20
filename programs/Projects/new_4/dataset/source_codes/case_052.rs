// 7. チャットルーム＋メッセージログ
use anchor_lang::prelude::*;
declare_id!("CHAT111122223333444455556666777788");

#[program]
pub mod misinit_chat_v6 {
    use super::*;

    pub fn init_room(
        ctx: Context<InitRoom>,
        topic: String,
    ) -> Result<()> {
        let r = &mut ctx.accounts.room;
        r.topic = topic;
        r.participants = Vec::new();
        Ok(())
    }

    pub fn join_room(
        ctx: Context<InitRoom>,
        user: Pubkey,
    ) -> Result<()> {
        let r = &mut ctx.accounts.room;
        r.participants.push(user);
        Ok(())
    }

    pub fn send_message(
        ctx: Context<InitRoom>,
        msg: String,
    ) -> Result<()> {
        let log = &mut ctx.accounts.message_log;
        log.messages.push(msg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRoom<'info> {
    #[account(init, payer = creator, space = 8 + (4+64) + (4+32*10))]
    pub room: Account<'info, RoomData>,
    #[account(mut)] pub message_log: Account<'info, MessageLog>,
    #[account(mut)] pub creator: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct RoomData { pub topic: String, pub participants: Vec<Pubkey> }
#[account]
pub struct MessageLog { pub messages: Vec<String> }
