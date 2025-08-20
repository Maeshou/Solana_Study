use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf085mvTWf");

#[program]
pub mod assign_resource_085 {
    use super::*;

    pub fn assign_resource(ctx: Context<Ctx085>) -> Result<()> {
        let old_id = ctx.accounts.entry.id;
        let bytes = ctx.accounts.caller.key().to_bytes();
        let new_id = Pubkey::new(&bytes[0..32]);
        ctx.accounts.entry.id = new_id;
        msg!("Case 085: ID changed from {} to {}", old_id, new_id);
        Ok(())
    }use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfESCROW");

#[program]
pub mod time_locked_escrow {
    use super::*;

    /// 管理者が資金をロックしてエスクローを初期化
    pub fn init_escrow(
        ctx: Context<InitEscrow>,
        release_ts: i64,
    ) -> Result<()> {
        let esc = &mut ctx.accounts.escrow;
        esc.creator    = ctx.accounts.creator.key();
        esc.beneficiary = ctx.accounts.beneficiary.key();
        esc.release_time = release_ts;
        esc.amount     = ctx.accounts.creator.to_account_info().lamports();
        msg!(
            "Escrow initialized: {} lamports locked until {} for {}",
            esc.amount,
            release_ts,
            esc.beneficiary
        );
        Ok(())
    }

    /// ベネフィシアリ（受益者）が署名付きで資金を受け取る
    pub fn release(
        ctx: Context<Release>,
    ) -> Result<()> {
        // 受益者署名チェック
        require!(
            ctx.accounts.beneficiary.is_signer,
            ErrorCode::Unauthorized
        );
        // 現在時刻取得
        let now = Clock::get()?.unix_timestamp;
        // 解放時刻チェック
        require!(
            now >= ctx.accounts.escrow.release_time,
            ErrorCode::TooEarly
        );
        // 預け入れ金を移転
        let esc_info = ctx.accounts.escrow.to_account_info();
        **esc_info.try_borrow_mut_lamports()? -= ctx.accounts.escrow.amount;
        **ctx.accounts.beneficiary.to_account_info().try_borrow_mut_lamports()? += ctx.accounts.escrow.amount;
        msg!(
            "{} lamports released to {}",
            ctx.accounts.escrow.amount,
            ctx.accounts.beneficiary.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    /// エスクロー用アカウントを初期化
    #[account(
        init,
        payer  = creator,
        space  = 8 + 32 + 32 + 8 + 8,
        seeds  = [b"escrow", creator.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,

    /// 受益者の Pubkey を設定（署名不要）
    pub beneficiary: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Release<'info> {
    #[account(
        mut,
        seeds = [b"escrow", escrow.creator.as_ref()],
        bump,
        has_one = beneficiary
    )]
    pub escrow: Account<'info, EscrowAccount>,

    /// 受益者署名必須
    pub beneficiary: Signer<'info>,
}

#[account]
pub struct EscrowAccount {
    pub creator:    Pubkey,
    pub beneficiary: Pubkey,
    pub release_time: i64,
    pub amount:     u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: signer required")]
    Unauthorized,
    #[msg("Too early: release time not reached")]
    TooEarly,
}

}

#[derive(Accounts)]
pub struct Ctx085<'info> {
    #[account(mut, has_one = admin)]
    pub entry: Account<'info, Entry085>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub caller: Signer<'info>,
}

#[account]
pub struct Entry085 {
    pub admin: Pubkey,
    pub id: Pubkey,
}
