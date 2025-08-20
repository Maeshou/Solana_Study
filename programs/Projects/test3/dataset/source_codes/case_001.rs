use anchor_lang::prelude::*;

declare_id!("Crst1111111111111111111111111111111111111");

#[program]
pub mod insecure_credit_reset {
    use super::*;

    pub fn reset_credit(ctx: Context<ResetCredit1>, credits: u64) -> Result<()> {
        let acct = &mut ctx.accounts.credit_account;
        // 多段処理でオリジナリティを追加
        let before = acct.credits;
        let computed = credits.checked_mul(1).unwrap();
        acct.credits = computed;
        acct.tag = if computed > before {
            "upgraded".to_string()
        } else {
            "downgraded".to_string()
        };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ResetCredit1<'info> {
    /// 再初期化リスクを含む init_if_needed
    #[account(
        init_if_needed,
        payer = fee_payer,
        space = 8 + 8 + 32 + 4,
        seeds = [b"credits", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub credit_account: Account<'info, CreditAccount1>,

    /// 署名チェックが欠如
    pub owner: UncheckedAccount<'info>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct CreditAccount1 {
    pub credits: u64,
    pub tag: String,
    pub owner: Pubkey,
}
