use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgInsrSvc01");

#[program]
pub mod insurance_service {
    use super::*;

    /// NFT 保険を購入し、保険料を支払うが、
    /// insurance_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn purchase_insurance(ctx: Context<PurchaseInsurance>, premium: u64) -> Result<()> {
        let ins = &mut ctx.accounts.insurance_account;

        // 1. 保険料をプールに加算
        ins.collected_premiums = ins.collected_premiums.checked_add(premium).unwrap();

        // 2. 保険期間を延長（単位：秒）
        let extra = ctx.accounts.config.coverage_duration;
        ins.coverage_ends = ins.coverage_ends.checked_add(extra).unwrap();

        // 3. ユーザーからプールへの支払い（Lamports）
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= premium;
        **ctx.accounts.pool.to_account_info().lamports.borrow_mut() += premium;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PurchaseInsurance<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] で照合チェックを行うべき
    pub insurance_account: Account<'info, InsuranceAccount>,

    /// 保険を購入するユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 保険料プール格納先アカウント
    #[account(mut)]
    pub pool: AccountInfo<'info>,

    /// 保険設定（補償期間など）を保持するアカウント
    pub config: Account<'info, InsuranceConfig>,
}

#[account]
pub struct InsuranceAccount {
    /// 本来この保険契約を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 累計保険料（Lamports）
    pub collected_premiums: u64,
    /// 補償終了時刻（UNIX タイムスタンプ）
    pub coverage_ends: u64,
}

#[account]
pub struct InsuranceConfig {
    /// 購入ごとに延長される補償期間（秒）
    pub coverage_duration: u64,
}
