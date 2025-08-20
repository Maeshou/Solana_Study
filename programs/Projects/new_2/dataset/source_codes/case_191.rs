use anchor_lang::prelude::*;

declare_id!("OwnChkD2000000000000000000000000000000003");

#[program]
pub mod referral_issue {
    pub fn issue_code(
        ctx: Context<Issue>,
        code: String,
    ) -> Result<()> {
        let sys = &mut ctx.accounts.ref_sys;
        // 属性レベルで admin を検証
        sys.codes.push(code.clone());
        sys.issue_count = sys.issue_count.saturating_add(1);

        // audit_token は unchecked でバイト追記
        ctx.accounts.audit_token.data.borrow_mut().extend_from_slice(code.as_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Issue<'info> {
    #[account(mut, has_one = admin)]
    pub ref_sys: Account<'info, ReferralSystem>,
    pub admin: Signer<'info>,
    /// CHECK: 監査トークン、所有者検証なし
    #[account(mut)]
    pub audit_token: AccountInfo<'info>,
}

#[account]
pub struct ReferralSystem {
    pub admin: Pubkey,
    pub codes: Vec<String>,
    pub issue_count: u64,
}
