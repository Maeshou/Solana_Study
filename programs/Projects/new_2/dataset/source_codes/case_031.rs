use anchor_lang::prelude::*;

declare_id!("ProgVault022IDXNEWLAYERBUMP98765");

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub bump: u8,
}

#[program]
pub mod case_022_vault {
    use super::*;

    /// PDA を初期化：authority と bump を格納
    pub fn initialize_vault(ctx: Context<InitVault>, bump: u8) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = *ctx.accounts.authority.key;
        vault.bump = bump;
        msg!("vault initialized for {}", ctx.accounts.authority.key());
        Ok(())
    }

    /// フルガード：PDA シードと has_one で authority を検証
    pub fn secure_credit(ctx: Context<SecureCredit>, amount: u64) -> Result<()> {
        let info = ctx.accounts.vault.to_account_info();
        **info.try_borrow_mut_lamports()? += amount;
        msg!("secure_credit +{}", amount);
        Ok(())
    }

    /// 部分ガード：PDA シードは検証するが has_one は省略
    pub fn partial_credit(ctx: Context<PartialCredit>, amount: u64) -> Result<()> {
        let info = ctx.accounts.vault.to_account_info();
        **info.try_borrow_mut_lamports()? += amount;
        msg!("partial_credit +{} (authority unchecked)", amount);
        Ok(())
    }

    /// 手動 PDA チェック：プログラム内で find_program_address を使って検証
    pub fn manual_pda_credit(ctx: Context<ManualPdaCredit>, amount: u64) -> Result<()> {
        let (pda, _bump) = Pubkey::find_program_address(
            &[b"vault", ctx.accounts.authority.key.as_ref()],
            ctx.program_id,
        );
        if pda == ctx.accounts.vault_pda.key() {
            **ctx.accounts.vault_pda.try_borrow_mut_lamports()? += amount;
            msg!("manual_pda_credit +{}", amount);
        } else {
            msg!("manual_pda_credit skipped: invalid PDA");
        }
        Ok(())
    }

    /// 完全未検証：生の AccountInfo を直接操作
    pub fn raw_credit(ctx: Context<RawCredit>, amount: u64) -> Result<()> {
        **ctx.accounts.free_vault.try_borrow_mut_lamports()? += amount;
        msg!("raw_credit +{}", amount);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitVault<'info> {
    #[account(
        init,
        seeds = [b"vault", authority.key.as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 1
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SecureCredit<'info> {
    #[account(
        mut,
        seeds = [b"vault", authority.key.as_ref()],
        bump = vault.bump,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct PartialCredit<'info> {
    #[account(
        mut,
        seeds = [b"vault", authority.key.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ManualPdaCredit<'info> {
    /// CHECK: プログラム内で PDA を手動検証
    #[account(mut)]
    pub vault_pda: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct RawCredit<'info> {
    /// CHECK: 完全未検証の生データ
    #[account(mut)]
    pub free_vault: AccountInfo<'info>,
}
