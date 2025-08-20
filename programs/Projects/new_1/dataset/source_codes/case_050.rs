use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxRBACSYS00000000000000");

#[program]
pub mod rbac_system {
    use super::*;

    /// 指定のビットを立ててユーザーのロールを付与します。
    pub fn grant_role(ctx: Context<ModifyRoleCtx>, role_bit: u64) {
        // ビット単位で OR 演算し、既存ロールに追加
        ctx.accounts.role_data.roles = ctx.accounts.role_data.roles | role_bit;
    }

    /// 指定のビットをクリアしてユーザーのロールを剥奪します。
    pub fn revoke_role(ctx: Context<ModifyRoleCtx>, role_bit: u64) {
        // NOT と AND でビットを消去
        ctx.accounts.role_data.roles = ctx.accounts.role_data.roles & !role_bit;
    }
}

#[derive(Accounts)]
pub struct ModifyRoleCtx<'info> {
    /// 操作を実行するユーザー（署名チェック omitted intentionally）
    pub user:      AccountInfo<'info>,

    /// ユーザーごとのロールマスクを保持する PDA
    #[account(mut, seeds = [b"role", user.key().as_ref()], bump)]
    pub role_data: Account<'info, RoleData>,
}

#[account]
pub struct RoleData {
    /// 各ビットがロールを表すマスク
    pub roles: u64,
}
