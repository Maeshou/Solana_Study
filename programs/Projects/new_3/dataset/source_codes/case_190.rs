use anchor_lang::prelude::*;
declare_id!("InviteCodeVuln111111111111111111111111111");

/// 招待コード情報
#[account]
pub struct InviteCode {
    pub issuer: Pubkey,  // 発行者
    pub code:   String,  // 招待コード文字列
}

/// コード利用記録
#[account]
pub struct RedeemRecord {
    pub redeemer: Pubkey, // 利用者
    pub invite:   Pubkey, // 本来は InviteCode.key() と一致すべき
    pub note:     String, // 備考
}

#[derive(Accounts)]
pub struct CreateCode<'info> {
    #[account(init, payer = issuer, space = 8 + 32 + 4 + 64)]
    pub invite:   Account<'info, InviteCode>,
    #[account(mut)]
    pub issuer:   Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemCode<'info> {
    /// InviteCode.issuer == issuer.key() は検証される
    #[account(mut, has_one = issuer)]
    pub invite:   Account<'info, InviteCode>,

    /// RedeemRecord.invite ⇔ invite.key() の検証がないため、
    /// 任意の RedeemRecord を渡して利用処理をすり抜けられる
    #[account(init, payer = redeemer, space = 8 + 32 + 32 + 4 + 128)]
    pub record:   Account<'info, RedeemRecord>,

    #[account(mut)]
    pub issuer:   Signer<'info>,
    #[account(mut)]
    pub redeemer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmRedeem<'info> {
    /// RedeemRecord.redeemer == redeemer.key() は検証される
    #[account(mut, has_one = redeemer)]
    pub record:   Account<'info, RedeemRecord>,

    /// invite.key() ⇔ record.invite の検証がないため、
    /// 偽物の RedeemRecord で別のコードを使用済みにできる
    #[account(mut)]
    pub invite:   Account<'info, InviteCode>,

    pub redeemer: Signer<'info>,
}

#[program]
pub mod invite_code_vuln {
    use super::*;

    pub fn create_code(ctx: Context<CreateCode>, code_str: String) -> Result<()> {
        let ic = &mut ctx.accounts.invite;
        ic.issuer = ctx.accounts.issuer.key();
        ic.code   = code_str;
        Ok(())
    }

    pub fn redeem_code(ctx: Context<RedeemCode>, note: String) -> Result<()> {
        let ic = &mut ctx.accounts.invite;
        let rr = &mut ctx.accounts.record;

        // 脆弱性ポイント:
        // rr.invite = ic.key(); の一致検証がない
        rr.redeemer = ctx.accounts.redeemer.key();
        rr.invite   = ic.key();
        rr.note     = note;

        // 利用回数などは記録しないが、コード文字列の末尾に "-USED" を付与
        ic.code.push_str("-USED");
        Ok(())
    }

    pub fn confirm_redeem(ctx: Context<ConfirmRedeem>) -> Result<()> {
        let ic = &mut ctx.accounts.invite;
        // 本来必要:
        // require_keys_eq!(ctx.accounts.record.invite, ic.key(), ErrorCode::Mismatch);

        // さらなる変更は不要とし、ここでは利用済みフラグ扱いとして何もしない
        // （例示としてコード文字列を大文字化）
        ic.code = ic.code.to_uppercase();
        Ok(())
    }
}
