use anchor_lang::prelude::*;
declare_id!("RoleMgmtVuln11111111111111111111111111111");

/// 役割（Role）情報
#[account]
pub struct Role {
    pub creator:        Pubkey,   // 役割発行者
    pub name:           String,   // 役割名
    pub assigned_count: u64,      // 割り当て回数
}

/// 役割割当記録
#[account]
pub struct AssignmentRecord {
    pub assignee:       Pubkey,   // 割り当て先ユーザー
    pub role:           Pubkey,   // 本来は Role.key() と一致すべき
    pub note:           String,   // 任意メモ
}

#[derive(Accounts)]
pub struct CreateRole<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 4 + 32 + 8)]
    pub role:       Account<'info, Role>,
    #[account(mut)]
    pub creator:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssignRole<'info> {
    /// Role.creator == creator.key() は検証される
    #[account(mut, has_one = creator)]
    pub role:       Account<'info, Role>,

    /// AssignmentRecord.role ⇔ role.key() の照合がないため、
    /// 任意の AssignmentRecord を渡して処理をすり抜けられる
    #[account(init, payer = creator, space = 8 + 32 + 32 + 4 + 64)]
    pub record:     Account<'info, AssignmentRecord>,

    #[account(mut)]
    pub creator:    Signer<'info>,
}

#[derive(Accounts)]
pub struct RevokeRole<'info> {
    /// AssignmentRecord.assignee == assignee.key() は検証される
    #[account(mut, has_one = assignee)]
    pub record:     Account<'info, AssignmentRecord>,

    /// role.key() ⇔ record.role の検証がないため、
    /// 偽物の AssignmentRecord で別の役割を解除できる
    #[account(mut)]
    pub role:       Account<'info, Role>,

    pub assignee:   Signer<'info>,
}

#[program]
pub mod role_mgmt_vuln {
    use super::*;

    pub fn create_role(ctx: Context<CreateRole>, name: String) -> Result<()> {
        let r = &mut ctx.accounts.role;
        r.creator        = ctx.accounts.creator.key();
        r.name           = name;
        r.assigned_count = 0;
        Ok(())
    }

    pub fn assign_role(ctx: Context<AssignRole>, assignee: Pubkey, note: String) -> Result<()> {
        let r = &mut ctx.accounts.role;
        let rec = &mut ctx.accounts.record;

        // 脆弱性ポイント:
        // rec.role = r.key(); の検証がない
        rec.assignee = assignee;
        rec.role     = r.key();
        rec.note     = note;

        // 単純に割り当てカウントを増やす（checked_add しない例）
        r.assigned_count = r.assigned_count + 1;
        Ok(())
    }

    pub fn revoke_role(ctx: Context<RevokeRole>) -> Result<()> {
        let r = &mut ctx.accounts.role;

        // 本来必要:
        // require_keys_eq!(ctx.accounts.record.role, r.key(), ErrorCode::Mismatch);

        // pop で最後に割り当てた件数を減らす（分岐・ループなし）
        let _ = r.assigned_count.checked_sub(1);
        Ok(())
    }
}
