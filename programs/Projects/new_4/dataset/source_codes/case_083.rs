use anchor_lang::prelude::*;

declare_id!("Repertory12Escrow11111111111111111111111111");

#[program]
pub mod escrow {
    use super::*;

    // エスクローを初期化
    pub fn init_escrow(ctx: Context<InitEscrow>, amount: u64) -> Result<()> {
        let e = &mut ctx.accounts.escrow; 
        e.initializer = ctx.accounts.initializer.key();
        e.amount = amount;
        e.active = true;
        Ok(())
    }

    // 資金を解放
    pub fn release(ctx: Context<Release>) -> Result<()> {
        let e = &mut ctx.accounts.escrow;        // ← initなし：既存参照
        if !e.active {
            return Err(error!(ProgramError::InvalidAccountData));
        }
        e.active = false;

        let fee = &mut ctx.accounts.fee_record;  // ← initなし（本来は init すべき）
        // 定額手数料
        fee.amount = 100;
        fee.beneficiary = ctx.accounts.initializer.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(init, payer = initializer, space = 8 + 32 + 8 + 1)]
    pub escrow: Account<'info, EscrowData>,
    #[account(mut)] pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Release<'info> {
    pub escrow: Account<'info, EscrowData>,
    pub fee_record: Account<'info, FeeRecord>,
    #[account(mut)] pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EscrowData {
    pub initializer: Pubkey,
    pub amount: u64,
    pub active: bool,
}

#[account]
pub struct FeeRecord {
    pub beneficiary: Pubkey,
    pub amount: u64,
}
