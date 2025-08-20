// ========================================
// 7. 脆弱なロイヤルティプログラム - Vulnerable Loyalty Program
// ========================================

use anchor_lang::prelude::*;

declare_id!("V7uLnErAbLeCoD3F0r3xAmP1e5tUdY7BaTt1eAr3nA6x");

#[program]
pub mod vulnerable_loyalty {
    use super::*;

    pub fn init_loyalty_program(ctx: Context<InitLoyaltyProgram>) -> Result<()> {
        let program = &mut ctx.accounts.loyalty_program;
        program.operator = ctx.accounts.operator.key();
        program.total_members = 0;
        program.points_issued = 0;
        program.tier_multiplier = 100; // 1.0x
        Ok(())
    }

    pub fn register_member(ctx: Context<RegisterMember>) -> Result<()> {
        let member = &mut ctx.accounts.member_account;
        member.program = ctx.accounts.loyalty_program.key();
        member.owner = ctx.accounts.owner.key();
        member.points_balance = 0;
        member.tier_level = 1;
        member.last_activity = Clock::get()?.unix_timestamp;

        let program = &mut ctx.accounts.loyalty_program;
        program.total_members = program.total_members.checked_add(1).unwrap_or(u64::MAX);
        Ok(())
    }

    // 脆弱性: assert_ne!と直接invoke使用
    pub fn vulnerable_points_transfer(ctx: Context<VulnerableTransfer>) -> Result<()> {
        let program = &mut ctx.accounts.loyalty_program;
        
        // 脆弱性: assert_ne!は簡単に回避可能
        assert_ne!(ctx.accounts.from_member.key(), ctx.accounts.to_member.key());
        
        // 脆弱性: UncheckedAccountで型安全性なし
        let from_data = ctx.accounts.from_member.try_borrow_mut_data()?;
        let to_data = ctx.accounts.to_member.try_borrow_mut_data()?;

        if from_data.len() >= 48 && to_data.len() >= 48 {
            // 脆弱性: discriminator検証なしでポイント残高操作
            let mut from_points_bytes = [0u8; 8];
            from_points_bytes.copy_from_slice(&from_data[40..48]);
            let mut from_points = u64::from_le_bytes(from_points_bytes);

            let mut to_points_bytes = [0u8; 8];
            to_points_bytes.copy_from_slice(&to_data[40..48]);
            let mut to_points = u64::from_le_bytes(to_points_bytes);

            // ポイント転送ループ
            for transfer_round in 0..5 {
                if from_points > 100 {
                    let transfer_amount = (from_points >> transfer_round) & 0x3F;
                    from_points = from_points.saturating_sub(transfer_amount);
                    to_points = to_points.checked_add(transfer_amount * 2).unwrap_or(u64::MAX); // 2倍ボーナス
                    
                    // ティア乗数調整
                    program.tier_multiplier = program.tier_multiplier.checked_add(transfer_round as u32 * 5).unwrap_or(500);
                    program.points_issued = program.points_issued.checked_add(transfer_amount).unwrap_or(u64::MAX);
                    
                    msg!("Transfer round {}: amount={}, bonus applied", transfer_round, transfer_amount);
                } else {
                    let tier_bonus = (program.tier_multiplier as u64 * to_points) / 100;
                    to_points = to_points.checked_add(tier_bonus).unwrap_or(u64::MAX);
                    
                    // ポイント発行上限突破
                    program.points_issued = program.points_issued.checked_add(tier_bonus * 3).unwrap_or(u64::MAX);
                    program.total_members = program.total_members.checked_add(transfer_round as u64).unwrap_or(u64::MAX);
                    
                    msg!("Tier bonus applied: {}", tier_bonus);
                }
            }

            // 脆弱性: 直接データ書き戻し
            let new_from_bytes = from_points.to_le_bytes();
            from_data[40..48].copy_from_slice(&new_from_bytes);
            
            let new_to_bytes = to_points.to_le_bytes();
            to_data[40..48].copy_from_slice(&new_to_bytes);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLoyaltyProgram<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 4)]
    pub loyalty_program: Account<'info, LoyaltyProgram>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterMember<'info> {
    #[account(mut)]
    pub loyalty_program: Account<'info, LoyaltyProgram>,
    #[account(init, payer = owner, space = 8 + 32 + 32 + 8 + 1 + 8)]
    pub member_account: Account<'info, MemberAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: assert_ne!とUncheckedAccount
#[derive(Accounts)]
pub struct VulnerableTransfer<'info> {
    #[account(mut)]
    pub loyalty_program: Account<'info, LoyaltyProgram>,
    /// CHECK: 脆弱性 - assert_ne!のみの検証
    pub from_member: UncheckedAccount<'info>,
    /// CHECK: 脆弱性 - 送金先検証不十分
    pub to_member: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

#[account]
pub struct LoyaltyProgram {
    pub operator: Pubkey,
    pub total_members: u64,
    pub points_issued: u64,
    pub tier_multiplier: u32,
}

#[account]
pub struct MemberAccount {
    pub program: Pubkey,
    pub owner: Pubkey,
    pub points_balance: u64,
    pub tier_level: u8,
    pub last_activity: i64,
}
