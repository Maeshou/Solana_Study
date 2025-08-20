use anchor_lang::prelude::*;

declare_id!("Cntc111111111111111111111111111111111111");

#[program]
pub mod contact_list {
    pub fn create_contact(
        ctx: Context<CreateContact>,
        name: String,
        phone: String,
    ) -> Result<()> {
        // 表示長チェック
        require!(
            name.len() <= 64,
            ErrorCode::NameTooLong
        );
        require!(
            phone.len() <= 32,
            ErrorCode::PhoneTooLong
        );

        let contact = &mut ctx.accounts.contact;
        // Signer Authorization & Owner Check
        contact.owner = ctx.accounts.user.key();
        contact.name  = name;
        contact.phone = phone;
        Ok(())
    }

    pub fn update_contact(
        ctx: Context<UpdateContact>,
        name: String,
        phone: String,
    ) -> Result<()> {
        let contact = &mut ctx.accounts.contact;
        // Account Matching + Signer Authorization
        require!(
            contact.owner == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );
        // 表示長チェック
        require!(
            name.len() <= 64,
            ErrorCode::NameTooLong
        );
        require!(
            phone.len() <= 32,
            ErrorCode::PhoneTooLong
        );
        contact.name  = name;
        contact.phone = phone;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateContact<'info> {
    /// init 制約で同じアカウントを二度初期化できない（Reinit Attack 防止）
    #[account(init, payer = user, space = 8 + 32 + 4 + 64 + 4 + 32)]
    pub contact: Account<'info, Contact>,

    /// このトランザクションを署名するユーザー
    #[account(mut)]
    pub user:    Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateContact<'info> {
    /// Account<Account> による Owner Check & Type Cosplay
    #[account(mut)]
    pub contact: Account<'info, Contact>,

    /// 実際に署名したユーザー
    pub user:    Signer<'info>,
}

#[account]
pub struct Contact {
    /// この連絡先を操作できるユーザー
    pub owner: Pubkey,
    /// 名前（最大 64 文字）
    pub name:  String,
    /// 電話番号（最大 32 文字）
    pub phone: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Name too long")]
    NameTooLong,
    #[msg("Phone number too long")]
    PhoneTooLong,
}
