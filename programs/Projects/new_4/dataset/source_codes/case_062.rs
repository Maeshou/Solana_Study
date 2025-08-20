// 7. チャットルーム＋ミュートリスト
use anchor_lang::prelude::*;
declare_id!("CHAT777788889999AAAABBBBCCCCDDDD");

#[program]
pub mod misinit_chatroom_v7 {
    use super::*;

    pub fn init_room(
        ctx: Context<InitRoom>,
        name: String,
    ) -> Result<()> {
        let r = &mut ctx.accounts.room;
        r.name = name;
        r.topic = String::new();
        Ok(())
    }

    pub fn set_topic(ctx: Context<InitRoom>, topic: String) -> Result<()> {
        let r = &mut ctx.accounts.room;
        r.topic = topic;
        Ok(())
    }

    pub fn mute_user(
        ctx: Context<InitRoom>,
        user: Pubkey,
    ) -> Result<()> {
        let log = &mut ctx.accounts.mute_log;
        log.muted.push(user);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRoom<'info> {
    #[account(init, payer = owner, space = 8 + (4+32) + (4+32) + 4)] pub room: Account<'info, RoomData>,
    #[account(mut)] pub mute_log: Account<'info, MuteLog>,
    #[account(mut)] pub owner: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct RoomData { pub name:String, pub topic:String, pub capacity:u32 }
#[account]
pub struct MuteLog { pub muted: Vec<Pubkey> }
