use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner015IDXUNIQUECASEAAA111");

#[program]
pub mod case_015_alternative {
    use super::*;

    /// Pattern A: Anchor-enforced vault (has_one + signer)
    pub fn enforced_increase(ctx: Context<EnforcedIncrease>, amount: u64) -> Result<()> {
        **ctx.accounts.secured.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("enforced_increase done");
        Ok(())
    }

    /// Pattern B: signer-only via inline closure (owner check skipped)
    pub fn signer_increase(ctx: Context<SignerIncrease>, amount: u64) -> Result<()> {
        (|| -> Result<()> {
            **ctx.accounts.unchecked.to_account_info().try_borrow_mut_lamports()? += amount;
            Ok(())
        })()?;
        msg!("signer_increase done");
        Ok(())
    }

    /// Pattern C: pair update (one checked, one unchecked)
    pub fn dual_increase(ctx: Context<DualIncrease>, amount: u64) -> Result<()> {
        [ ctx.accounts.secured.to_account_info(), ctx.accounts.unchecked.to_account_info() ]
            .iter()
            .for_each(|acct| { **acct.try_borrow_mut_lamports().unwrap() += amount; });
        msg!("dual_increase done");
        Ok(())
    }

    /// Pattern D: all remaining_accounts updated
    pub fn batch_increase(ctx: Context<BatchIncrease>, amount: u64) -> Result<()> {
        ctx.remaining_accounts
            .iter()
            .for_each(|acct| **acct.try_borrow_mut_lamports().unwrap() += amount);
        msg!("batch_increase done");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnforcedIncrease<'info> {
    #[account(mut, has_one = authority)]
    secured: Account<'info, Vault>,
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignerIncrease<'info> {
    /// CHECK: no owner validation
    #[account(mut)]
    unchecked: AccountInfo<'info>,
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DualIncrease<'info> {
    #[account(mut, has_one = authority)]
    secured: Account<'info, Vault>,
    /// CHECK: raw account, skip ownership
    #[account(mut)]
    unchecked: AccountInfo<'info>,
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchIncrease<'info> {
    authority: Signer<'info>,
    system_program: Program<'info, System>,
    // any number of additional vault accounts passed as remaining_accounts
}

#[account]
pub struct Vault {
    authority: Pubkey,
}
