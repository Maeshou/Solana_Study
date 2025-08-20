use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner011IDXABCDEF0112233445");

#[program]
pub mod case_011_mixture {
    use super::*;

    /// フルガード：has_one と signer でオーナーと署名を検証
    pub fn reserved_credit(ctx: Context<ReservedCredit>, amount: u64) -> Result<()> {
        **ctx.accounts.vault_acc.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("reserved_credit executed");
        Ok(())
    }

    /// 署名のみ検証：AccountInfo を直接操作、オーナー検証は省略
    pub fn signer_only_credit(ctx: Context<SignerOnlyCredit>, amount: u64) -> Result<()> {
        **ctx.accounts.raw_acc.try_borrow_mut_lamports()? += amount;
        msg!("signer_only_credit executed");
        Ok(())
    }

    /// オーナーのみ検証：has_one による検証後、to_account_info() で加算
    pub fn owner_only_credit(ctx: Context<OwnerOnlyCredit>, amount: u64) -> Result<()> {
        let acct = ctx.accounts.vault.to_account_info();
        **acct.try_borrow_mut_lamports()? += amount;
        msg!("owner_only_credit executed");
        Ok(())
    }

    /// 無制限アクセス：誰でも実行可能
    pub fn unrestricted_credit(ctx: Context<UnrestrictedCredit>, amount: u64) -> Result<()> {
        **ctx.accounts.any_acc.try_borrow_mut_lamports()? += amount;
        msg!("unrestricted_credit executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReservedCredit<'info> {
    #[account(mut, has_one = authority)]
    pub vault_acc: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignerOnlyCredit<'info> {
    #[account(mut)]
    pub raw_acc: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OwnerOnlyCredit<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    /// CHECK: AccountInfo なので署名不要
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnrestrictedCredit<'info> {
    #[account(mut)]
    pub any_acc: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
}
