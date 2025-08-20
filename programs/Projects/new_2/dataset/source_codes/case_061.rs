use anchor_lang::prelude::*;
use std::iter;

declare_id!("ProgMixedOwner032IDXNOVELCASE032XYZ");

#[program]
pub mod case_032_novel {
    use super::*;

    /// モードα：has_one + signer で検証済みVaultをインクリメント
    pub fn alpha_increment(ctx: Context<AlphaCtx>, delta: u64) -> Result<()> {
        let acct = ctx.accounts.secure_vault.to_account_info();
        let before = acct.lamports();
        **acct.try_borrow_mut_lamports()? = before + delta;
        msg!("alpha_increment: {}→{}", before, before + delta);
        Ok(())
    }

    /// モードβ：署名のみチェック、chain で複数アカウントをまとめて操作（ownerチェックなし）
    pub fn beta_chain(ctx: Context<BetaCtx>, delta: u64) -> Result<()> {
        iter::once(&ctx.accounts.raw_one)
            .chain(iter::once(&ctx.accounts.raw_two))
            .for_each(|info| {
                let _ = info.try_borrow_mut_lamports().map(|mut lam| *lam += delta);
            });
        msg!("beta_chain completed");
        Ok(())
    }

    /// モードγ：手動オーナーチェック → early returnスタイル
    pub fn gamma_guard(ctx: Context<GammaCtx>, delta: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        if vault.owner != ctx.accounts.user.key() {
            msg!("gamma_guard: unauthorized");
            return Ok(())
        }
        let info = vault.to_account_info();
        let bal = info.lamports();
        **info.try_borrow_mut_lamports()? = bal + delta;
        msg!("gamma_guard applied: {}", bal + delta);
        Ok(())
    }

    /// モードδ：remaining_accounts に渡されたそれぞれに1.5倍（四捨五入）で分配
    pub fn delta_share(ctx: Context<DeltaCtx>, base: u64) -> Result<()> {
        let bonus = ((base as f64) * 1.5).round() as u64;
        for acct in &ctx.remaining_accounts {
            **acct.try_borrow_mut_lamports()? += bonus;
        }
        msg!("delta_share: +{} each", bonus);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AlphaCtx<'info> {
    #[account(mut, has_one = authority)]
    pub secure_vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BetaCtx<'info> {
    /// CHECK: raw AccountInfo は owner チェックなし
    #[account(mut)]
    pub raw_one: AccountInfo<'info>,
    #[account(mut)]
    pub raw_two: AccountInfo<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GammaCtx<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeltaCtx<'info> {
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意のアカウントを受け取る
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}
