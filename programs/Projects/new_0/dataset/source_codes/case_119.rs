use anchor_lang::prelude::*;

declare_id!("GrpM1111111111111111111111111111111111111");

#[program]
pub mod group_manager {
    /// 新しいグループを作成
    pub fn create_group(
        ctx: Context<CreateGroup>,
        name: String,
    ) -> Result<()> {
        // グループ名長チェック
        if name.len() > 64 {
            return Err(ErrorCode::NameTooLong.into());
        }

        let grp = &mut ctx.accounts.group;
        grp.admin        = ctx.accounts.user.key();
        grp.name         = name;
        grp.participants = Vec::new();
        Ok(())
    }

    /// グループに参加
    pub fn join_group(ctx: Context<JoinGroup>) -> Result<()> {
        let grp = &mut ctx.accounts.group;
        let user = ctx.accounts.user.key();

        // すでにメンバーなら拒否
        if grp.participants.iter().any(|&p| p == user) {
            return Err(ErrorCode::AlreadyMember.into());
        }
        // 最大メンバー数チェック
        if grp.participants.len() >= 10 {
            return Err(ErrorCode::GroupFull.into());
        }
        grp.participants.push(user);
        Ok(())
    }

    /// グループから退会
    pub fn leave_group(ctx: Context<LeaveGroup>) -> Result<()> {
        let grp = &mut ctx.accounts.group;
        let user = ctx.accounts.user.key();

        // メンバーでなければ拒否
        if !grp.participants.iter().any(|&p| p == user) {
            return Err(ErrorCode::NotMember.into());
        }
        // 退会処理
        grp.participants.retain(|&p| p != user);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateGroup<'info> {
    /// 同一アカウント再初期化防止（Reinit Attack）
    #[account(init, payer = user, space = 8 + 32 + 4 + 64 + 4 + (10 * 32))]
    pub group:    Account<'info, GroupAccount>,

    /// グループ作成者（署名必須）
    #[account(mut)]
    pub user:     Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinGroup<'info> {
    /// Account<> による Owner Check / Type Cosplay
    #[account(mut)]
    pub group:    Account<'info, GroupAccount>,

    /// 参加者（署名必須）
    pub user:     Signer<'info>,
}

#[derive(Accounts)]
pub struct LeaveGroup<'info> {
    #[account(mut)]
    pub group:    Account<'info, GroupAccount>,
    pub user:     Signer<'info>,
}

#[account]
pub struct GroupAccount {
    /// グループ管理者
    pub admin:        Pubkey,
    /// グループ名（最大64文字）
    pub name:         String,
    /// 参加者リスト（最大10名）
    pub participants: Vec<Pubkey>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Group name is too long")]
    NameTooLong,
    #[msg("Already a member")]
    AlreadyMember,
    #[msg("Not a member")]
    NotMember,
    #[msg("Group is full")]
    GroupFull,
}
