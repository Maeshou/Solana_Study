use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("AprvlMgr48484848484848484848484848484848");

#[program]
pub mod approval_mgr48 {
    use super::*;

    /// ユーザ自身を「承認済み」に設定
    pub fn sign(ctx: Context<SignApproval>) -> Result<()> {
        let m = &mut ctx.accounts.manager;
        m.map.insert(ctx.accounts.user.key(), true);
        Ok(())
    }

    /// ユーザ自身の承認を解除
    pub fn revoke(ctx: Context<SignApproval>) -> Result<()> {
        let m = &mut ctx.accounts.manager;
        m.map.insert(ctx.accounts.user.key(), false);
        Ok(())
    }

    /// 指定ユーザの承認状態を返す
    pub fn is_approved(
        ctx: Context<ViewApproval>,
        target: Pubkey,
    ) -> Result<ApprovalView> {
        let m = &ctx.accounts.manager;
        let approved = m.map.get(&target).copied().unwrap_or(false);
        Ok(ApprovalView { approved })
    }
}

#[derive(Accounts)]
pub struct SignApproval<'info> {
    #[account(mut)]
    pub manager: Account<'info, ApprovalData>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewApproval<'info> {
    pub manager: Account<'info, ApprovalData>,
}

#[account]
pub struct ApprovalData {
    pub map: BTreeMap<Pubkey, bool>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ApprovalView {
    pub approved: bool,
}
