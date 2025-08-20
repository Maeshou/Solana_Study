// ========================================
// 8. 脆弱なマルチシグ - Vulnerable Multisig
// ========================================

use anchor_lang::prelude::*;

declare_id!("V8uLnErAbLeCoD3F0r3xAmP1e5tUdY7BaTt1eAr3nA7x");

#[program]
pub mod vulnerable_multisig {
    use super::*;

    pub fn init_multisig(ctx: Context<InitMultisig>, threshold: u8) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;
        multisig.threshold = threshold;
        multisig.signer_count = 0;
        multisig.nonce = 0;
        multisig.executed_txs = 0;
        Ok(())
    }

    pub fn add_signer(ctx: Context<AddSigner>, signer_key: Pubkey) -> Result<()> {
        let signer_record = &mut ctx.accounts.signer_record;
        signer_record.multisig = ctx.accounts.multisig.key();
        signer_record.signer = signer_key;
        signer_record.approved_count = 0;
        signer_record.active = true;

        let multisig = &mut ctx.accounts.multisig;
        multisig.signer_count = multisig.signer_count.checked_add(1).unwrap_or(255);
        Ok(())
    }

    // 脆弱性: 直接invokesとAccountInfo併用
    pub fn vulnerable_execute(ctx: Context<VulnerableExecute>) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;
        
        // 脆弱性: AccountInfoで署名者検証不十分
        let signer1_info = &ctx.accounts.signer1;
        let signer2_info = &ctx.accounts.signer2;
        let signer3_info = &ctx.accounts.signer3;

        // 脆弱性: lamportsのみで署名者判定
        let sig1_weight = **signer1_info.lamports.borrow() / 1_000_000; // SOL to weight
        let sig2_weight = **signer2_info.lamports.borrow() / 1_000_000;
        let sig3_weight = **signer3_info.lamports.borrow() / 1_000_000;

        // マルチシグ実行ループ
        for execution_round in 0..6 {
            if sig1_weight > sig2_weight + sig3_weight {
                // 署名者1主導の実行
                let execution_power = sig1_weight * (execution_round + 1);
                multisig.executed_txs = multisig.executed_txs.checked_add(execution_power).unwrap_or(u64::MAX);
                
                // ナンス管理（ビット操作）
                multisig.nonce = (multisig.nonce << 1) | 0x01;
                multisig.threshold = (multisig.threshold + execution_round as u8).min(10);
                
                msg!("Signer1 execution round {}: power={}", execution_round, execution_power);
            } else {
                // 複数署名者の組み合わせ実行
                let combined_weight = sig2_weight ^ sig3_weight;
                let weighted_execution = combined_weight * (execution_round + 2);
                multisig.executed_txs = multisig.executed_txs.checked_add(weighted_execution).unwrap_or(u64::MAX);
                
                // 閾値動的調整
                let threshold_adjustment = (combined_weight >> 2) & 0x07;
                multisig.threshold = (multisig.threshold + threshold_adjustment as u8).min(255);
                multisig.signer_count = multisig.signer_count.checked_add(execution_round as u8).unwrap_or(255);
                
                msg!("Combined execution round {}: weight={}", execution_round, combined_weight);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMultisig<'info> {
    #[account(init, payer = creator, space = 8 + 1 + 1 + 8 + 8)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddSigner<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(init, payer = admin, space = 8 + 32 + 32 + 4 + 1)]
    pub signer_record: Account<'info, SignerRecord>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: AccountInfoで署名者検証回避
#[derive(Accounts)]
pub struct VulnerableExecute<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    /// CHECK: 脆弱性 - 署名者1検証不十分
    pub signer1: AccountInfo<'info>,
    /// CHECK: 脆弱性 - 署名者2検証不十分
    pub signer2: AccountInfo<'info>,  
    /// CHECK: 脆弱性 - 署名者3検証不十分
    pub signer3: AccountInfo<'info>,
    pub executor: Signer<'info>,
}

#[account]
pub struct Multisig {
    pub threshold: u8,
    pub signer_count: u8,
    pub nonce: u64,
    pub executed_txs: u64,
}

#[account]
pub struct SignerRecord {
    pub multisig: Pubkey,
    pub signer: Pubkey,
    pub approved_count: u32,
    pub active: bool,
}