use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgChatSvc002");

#[program]
pub mod chat_service {
    use super::*;

    /// チャネルのタイトルを更新するが、
    /// ownership マッチング検証がないため任意のチャネルを操作可能
    pub fn update_channel_title(ctx: Context<UpdateChannelTitle>, new_title: String) -> Result<()> {
        let channel = &mut ctx.accounts.channel;
        // ↓ 本来は channel.owner と ctx.accounts.user.key() の一致を検証すべき
        channel.title = new_title;
        Ok(())
    }

    /// チャネルのトピックを更新するが、
    /// ownership マッチング検証がないため任意のチャネルを操作可能
    pub fn update_channel_topic(ctx: Context<UpdateChannelTopic>, new_topic: String) -> Result<()> {
        let channel = &mut ctx.accounts.channel;
        // ↓ 本来は channel.owner と ctx.accounts.user.key() の一致を検証すべき
        channel.topic = new_topic;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateChannelTitle<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合を行うべき
    pub channel: Account<'info, Channel>,
    /// タイトル更新をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateChannelTopic<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合を行うべき
    pub channel: Account<'info, Channel>,
    /// トピック更新をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct Channel {
    /// このチャネルを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// チャネルのタイトル
    pub title: String,
    /// チャネルのトピック
    pub topic: String,
}
