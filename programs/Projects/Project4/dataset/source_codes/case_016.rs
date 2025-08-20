use anchor_lang::prelude::*;

declare_id!("Var4Msg4444444444444444444444444444444444");

#[program]
pub mod varied_message {
    use super::*;

    pub fn init_chat(ctx: Context<InitChat>) -> Result<()> {
        let c = &mut ctx.accounts.chat;
        c.count = 0;
        Ok(())
    }

    pub fn broadcast(ctx: Context<Broadcast>, messages: Vec<String>) -> Result<()> {
        let mut sent = ctx.accounts.chat.count;
        
        for msg in messages.iter() {
            // 単一条件（msg.len() > 0）のみ
            if msg.len() > 0 {
                sent += 1;
            }
        }
        
        let nc = &mut ctx.accounts.new_chat;
        nc.sent = sent;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitChat<'info> {
    #[account(init, payer = user, space = 8 + 4)]
    pub chat: Account<'info, ChatData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Broadcast<'info> {
    pub chat: Account<'info, ChatData>,
    #[account(mut, init, payer = user, space = 8 + 4)]
    pub new_chat: Account<'info, NewChatData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ChatData {
    pub count: u32,
}

#[account]
pub struct NewChatData {
    pub sent: u32,
}
