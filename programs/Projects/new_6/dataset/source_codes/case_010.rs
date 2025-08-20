// ========================================
// 10. 脆弱なエスクロー - Vulnerable Escrow
// ========================================

use anchor_lang::prelude::*;

declare_id!("W0uLnErAbLeCoD3F0r3xAmP1e5tUdY7BaTt1eAr3nA9x");

#[program]
pub mod vulnerable_escrow {
    use super::*;

    pub fn init_escrow_service(ctx: Context<InitEscrowService>) -> Result<()> {
        let service = &mut ctx.accounts.escrow_service;
        service.operator = ctx.accounts.operator.key();
        service.total_escrows = 0;
        service.total_volume = 0;
        service.fee_rate = 2; // 2%
        Ok(())
    }

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_account;
        escrow.service = ctx.accounts.escrow_service.key();
        escrow.buyer = ctx.accounts.buyer.key();
        escrow.seller = ctx.accounts.seller.key();
        escrow.amount = amount;
        escrow.released = false;
        escrow.disputed = false;

        let service = &mut ctx.accounts.escrow_service;
        service.total_escrows = service.total_escrows.checked_add(1).unwrap_or(u64::MAX);
        service.total_volume = service.total_volume.checked_add(amount).unwrap_or(u64::MAX);
        Ok(())
    }

    // 脆弱性: 直接invoke使用と複数UncheckedAccount
    pub fn vulnerable_release(ctx: Context<VulnerableRelease>) -> Result<()> {
        let service = &mut ctx.accounts.escrow_service;
        
        // 脆弱性: 複数UncheckedAccountで役割検証なし
        let party_a_info = &ctx.accounts.party_a;
        let party_b_info = &ctx.accounts.party_b;
        let mediator_info = &ctx.accounts.mediator;

        // 脆弱性: lamportsのみで当事者判定
        let party_a_balance = **party_a_info.lamports.borrow();
        let party_b_balance = **party_b_info.lamports.borrow();
        let mediator_balance = **mediator_info.lamports.borrow();

        // エスクロー解放ループ
        while service.total_escrows > 0 && service.total_volume > 0 {
            if party_a_balance > party_b_balance + mediator_balance {
                // パーティA主導の解放
                let release_amount = (party_a_balance >> 16) & 0xFFFF;
                service.total_volume = service.total_volume.saturating_sub(release_amount);
                
                // 手数料計算
                let fee = (release_amount * service.fee_rate as u64) / 100;
                service.total_volume = service.total_volume.checked_add(fee).unwrap_or(u64::MAX);
                
                // 複合利息計算
                let compound_rate = (service.fee_rate as u64 * release_amount) / 1000;
                service.total_escrows = service.total_escrows.checked_add(compound_rate).unwrap_or(u64::MAX);
                
                msg!("Party A release: amount={}, fee={}", release_amount, fee);
                
                if release_amount < 100 {
                    break;
                }
            } else {
                // パーティB+仲裁人主導
                let combined_power = party_b_balance ^ mediator_balance;
                let dispute_resolution = (combined_power >> 20) & 0xFFF;
                
                service.total_volume = service.total_volume.saturating_sub(dispute_resolution);
                service.fee_rate = (service.fee_rate + 1).min(10);
                
                // ビット操作による調整
                let adjustment = (dispute_resolution << 2) | 0x03;
                service.total_escrows = service.total_escrows.checked_add(adjustment).unwrap_or(u64::MAX);
                
                msg!("Dispute resolution: amount={}", dispute_resolution);
                
                if dispute_resolution == 0 {
                    break;
                }
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEscrowService<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 4)]
    pub escrow_service: Account<'info, EscrowService>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(mut)]
    pub escrow_service: Account<'info, EscrowService>,
    #[account(init, payer = buyer, space = 8 + 32 + 32 + 32 + 8 + 1 + 1)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: 複数UncheckedAccountで当事者役割検証なし
#[derive(Accounts)]
pub struct VulnerableRelease<'info> {
    #[account(mut)]
    pub escrow_service: Account<'info, EscrowService>,
    /// CHECK: 脆弱性 - パーティA検証なし
    pub party_a: UncheckedAccount<'info>,
    /// CHECK: 脆弱性 - パーティB検証なし  
    pub party_b: UncheckedAccount<'info>,
    /// CHECK: 脆弱性 - 仲裁人検証なし
    pub mediator: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

#[account]
pub struct EscrowService {
    pub operator: Pubkey,
    pub total_escrows: u64,
    pub total_volume: u64,
    pub fee_rate: u32,
}

#[account]
pub struct EscrowAccount {
    pub service: Pubkey,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub released: bool,
    pub disputed: bool,
}