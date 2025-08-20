use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzV2");

#[program]
pub mod event_checkin {
    use super::*;

    /// イベント出欠リストを作成：主催者のみ操作可能
    pub fn create_list(ctx: Context<CreateList>) -> Result<()> {
        let list = &mut ctx.accounts.list;
        list.organizer = ctx.accounts.organizer.key();
        list.attendees.clear();
        Ok(())
    }

    /// データレス「チケット」アカウントの存在・署名でチェックインを許可
    pub fn check_in(ctx: Context<CheckIn>) -> Result<()> {
        let list = &mut ctx.accounts.list;
        // データは無視、キーだけ attendee として記録
        list.attendees.push(ctx.accounts.ticket.key());
        Ok(())
    }
}

#[account]
pub struct AttendanceList {
    pub organizer:  Pubkey,       // リスト作成者
    pub attendees:  Vec<Pubkey>,  // チェックイン済ユーザー（ticket キー）
}

#[derive(Accounts)]
pub struct CreateList<'info> {
    /// AttendanceList を初期化
    #[account(
        init,
        payer = organizer,
        space = 8   // discriminator
              + 32  // organizer
              + 4 + 32 * 100  // attendees: max 100 entries
    )]
    pub list:      Account<'info, AttendanceList>,

    /// イベント主催者（署名必須）
    #[account(mut)]
    pub organizer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckIn<'info> {
    /// 既存の出欠リスト（主催者のみチェックイン受付）
    #[account(
        mut,
        has_one = organizer
    )]
    pub list:      Account<'info, AttendanceList>,

    /// 有効な“チケット”として振る舞うデータレスアカウント  
    /// - SystemProgram 所有 (`owner = System::id()`)、  
    /// - かつそのキーの署名が必要 (`signer`)  
    #[account(
        signer,
        constraint = ticket.to_account_info().owner == &System::id()
    )]
    pub ticket:    AccountInfo<'info>,

    /// リスト所有者の署名でのみチェックインを許可
    #[account(signer)]
    pub organizer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
