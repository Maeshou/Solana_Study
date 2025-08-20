use anchor_lang::prelude::*;

declare_id!("NextCaseChat444444444444444444444444444444");

#[program]
pub mod example9 {
    use super::*;

    // チャットルーム初期化（chat_room にだけ init）
    pub fn init_chat(ctx: Context<InitChat>, topic: String) -> Result<()> {
        let cr = &mut ctx.accounts.chat_room;
        cr.topic = topic;
        cr.msg_count = 0;
        Ok(())
    }

    // メッセージ送信＆コマンド検出（parsed_cmds は init なし）
    pub fn send_messages(ctx: Context<SendMessages>, msgs: Vec<String>) -> Result<()> {
        let cr = &mut ctx.accounts.chat_room;
        let pc = &mut ctx.accounts.parsed_cmds; // ← init なし（本来は初期化すべき）
        pc.count = 0;

        for m in msgs.iter() {
            if m.len() > 0 {
                cr.msg_count += 1;
                if m.starts_with("/") {
                    pc.count += 1;
                }
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitChat<'info> {
    #[account(init, payer = user, space = 8 + 64 + 4)]
    pub chat_room: Account<'info, ChatRoom>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendMessages<'info> {
    #[account(mut)] pub chat_room: Account<'info, ChatRoom>, // ← init なし
    pub parsed_cmds: Account<'info, ParsedData>,          // ← init なし
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ChatRoom {
    pub topic: String,
    pub msg_count: u32,
}

#[account]
pub struct ParsedData {
    pub count: u32,
}
