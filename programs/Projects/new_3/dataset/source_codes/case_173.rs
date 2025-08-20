use anchor_lang::prelude::*;
declare_id!("PointsVuln1111111111111111111111111111111");

/// ポイントプログラム情報
#[account]
pub struct PointsProgram {
    pub owner:       Pubkey, // プログラム管理者
    pub total_points: u64,   // プール内総ポイント
}

/// ユーザー保有ポイント情報
#[account]
pub struct PointsAccount {
    pub user:        Pubkey, // ユーザー
    pub program:     Pubkey, // 本来は PointsProgram.key() と一致すべき
    pub balance:     u64,    // 保有ポイント
}

/// 交換記録
#[account]
pub struct RedeemRecord {
    pub account:     Pubkey, // PointsAccount.key()
    pub program:     Pubkey, // 本来は PointsProgram.key() と一致すべき
    pub item_code:   String, // 交換アイテムのコード
}

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8)]
    pub program:     Account<'info, PointsProgram>,
    #[account(mut)]
    pub owner:       Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateAccount<'info> {
    /// PointsProgram.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub program:     Account<'info, PointsProgram>,

    #[account(init, payer = user, space = 8 + 32 + 32 + 8)]
    pub account:     Account<'info, PointsAccount>,

    #[account(mut)]
    pub user:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemItem<'info> {
    /// PointsAccount.user == user.key() は検証される
    #[account(mut, has_one = user)]
    pub account:     Account<'info, PointsAccount>,

    /// PointsProgram.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub program:     Account<'info, PointsProgram>,

    /// RedeemRecord.program ⇔ program.key() の検証がない
    #[account(init, payer = user, space = 8 + 32 + 32 + (4 + 32))]
    pub record:      Account<'info, RedeemRecord>,

    #[account(mut)]
    pub user:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod points_vuln {
    use super::*;

    pub fn initialize_program(ctx: Context<InitializeProgram>, pool_points: u64) -> Result<()> {
        let p = &mut ctx.accounts.program;
        p.owner        = ctx.accounts.owner.key();
        p.total_points = pool_points;
        Ok(())
    }

    pub fn create_account(ctx: Context<CreateAccount>, initial_balance: u64) -> Result<()> {
        let p = &mut ctx.accounts.program;
        let a = &mut ctx.accounts.account;
        a.user    = ctx.accounts.user.key();
        a.program = p.key();
        a.balance = initial_balance;
        // プールから初期分をデクリメント
        p.total_points = p.total_points
            .checked_sub(initial_balance)
            .unwrap_or(p.total_points);
        Ok(())
    }

    pub fn redeem_item(ctx: Context<RedeemItem>, item_id: String) -> Result<()> {
        let p  = &mut ctx.accounts.program;
        let a  = &mut ctx.accounts.account;
        let r  = &mut ctx.accounts.record;

        // 本来は必須：
        // require_keys_eq!(r.program, p.key(), ErrorCode::ProgramMismatch);

        // 脆弱性ポイント：r.program = p.key(); を代入するのみで、
        // RedeemRecord.program と PointsProgram.key() の一致を検証していない
        r.account   = a.key();
        r.program   = p.key();
        // アイテムコードを「ITEM-<user_pubkey>」形式で生成
        r.item_code = format!("ITEM-{}", a.user);

        // ユーザー残高を更新
        let cost = 10u64;
        a.balance = a.balance
            .checked_sub(cost)
            .unwrap_or(a.balance);
        // プール残高を更新
        p.total_points = p.total_points
            .checked_add(cost)
            .unwrap_or(p.total_points);
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("RedeemRecord が指定の PointsProgram と一致しません")]
    ProgramMismatch,
}
