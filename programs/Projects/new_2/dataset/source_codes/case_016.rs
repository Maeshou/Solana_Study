use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner019IDXECLECTICCASE019ABC");

#[program]
pub mod case_019_eclectic {
    use super::*;

    /// Pattern Alpha: Anchor-enforced via has_one + signer
    pub fn alpha_credit(ctx: Context<AlphaCredit>, amount: u64) -> Result<()> {
        let info = ctx.accounts.vault_acc.to_account_info();
        let before = **info.try_borrow_lamports()?;
        let updated = before + amount;
        **info.try_borrow_mut_lamports()? = updated;
        msg!("alpha_credit: {}→{}", before, updated);
        Ok(())
    }

    /// Pattern Beta: signer-only via while loop (owner unchecked)
    pub fn beta_credit(ctx: Context<BetaCredit>, amount: u64) -> Result<()> {
        let raw = &ctx.accounts.raw_acc;
        let mut balance = **raw.try_borrow_lamports()?;
        let mut i = 0;
        while i < 1 {
            balance += amount;
            i += 1;
        }
        **raw.try_borrow_mut_lamports()? = balance;
        msg!("beta_credit: {}", balance);
        Ok(())
    }

    /// Pattern Gamma: manual owner check via if (signer optional)
    pub fn gamma_credit(ctx: Context<GammaCredit>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_meta;
        let caller = ctx.accounts.user_meta.key();
        if vault.owner == caller {
            let info = vault.to_account_info();
            let before = **info.try_borrow_lamports()?;
            **info.try_borrow_mut_lamports()? = before + amount;
            msg!("gamma_credit applied");
        }
        Ok(())
    }

    /// Pattern Delta: index-based update of remaining_accounts
    pub fn delta_credit(ctx: Context<DeltaCredit>, amount: u64) -> Result<()> {
        let len = ctx.remaining_accounts.len();
        for idx in 0..len {
            let acct = &ctx.remaining_accounts[idx];
            let bal = acct.lamports();
            **acct.try_borrow_mut_lamports()? = bal + amount;
        }
        msg!("delta_credit done");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AlphaCredit<'info> {
    #[account(mut, has_one = authority)]
    pub vault_acc: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BetaCredit<'info> {
    #[account(mut)]
    pub raw_acc: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GammaCredit<'info> {
    #[account(mut)]
    pub vault_meta: Account<'info, Vault>,
    /// CHECK: 比較用の AccountInfo（署名不要）
    pub user_meta: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeltaCredit<'info> {
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意の追加アカウントを受け取る
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}
