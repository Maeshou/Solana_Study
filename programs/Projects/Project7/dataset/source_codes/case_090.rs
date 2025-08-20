// 03. ギルド会費徴収・返金プログラム
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("Guild3333333333333333333333333333333333333");

#[program]
pub mod guild_fees {
    use super::*;

    pub fn init_guild_treasury(
        ctx: Context<InitGuildTreasury>,
        monthly_fee: u64,
        refund_period_slots: u64,
    ) -> Result<()> {
        let treasury = &mut ctx.accounts.guild_treasury;
        treasury.guild_leader = ctx.accounts.guild_leader.key();
        treasury.fee_token_mint = ctx.accounts.fee_mint.key();
        treasury.monthly_fee = monthly_fee;
        treasury.refund_period_slots = refund_period_slots;
        treasury.total_collected = 0;
        treasury.total_refunded = 0;
        treasury.member_count = 0;
        treasury.treasury_status = TreasuryStatus::Open;
        Ok(())
    }

    pub fn process_fee_collection(ctx: Context<ProcessFeeCollection>) -> Result<()> {
        let treasury = &mut ctx.accounts.guild_treasury;
        let current_slot = Clock::get()?.slot;
        
        if treasury.treasury_status == TreasuryStatus::Closed {
            return Ok(());
        }

        let fee_amount = treasury.monthly_fee;
        let member_data = &mut ctx.accounts.member_data;
        
        // 未払い期間チェック
        if member_data.last_payment_slot + 432000 < current_slot { // 約30日相当
            member_data.payment_status = PaymentStatus::Overdue;
        }

        if member_data.payment_status == PaymentStatus::Current {
            return Ok(());
        }

        // 会費徴収
        transfer(
            ctx.accounts.collect_fee_ctx(),
            fee_amount,
        )?;

        member_data.last_payment_slot = current_slot;
        member_data.payment_status = PaymentStatus::Current;
        member_data.total_paid += fee_amount;
        
        treasury.total_collected += fee_amount;
        
        // 初回メンバーカウント
        if member_data.is_first_payment {
            treasury.member_count += 1;
            member_data.is_first_payment = false;
        }

        // メンバー数上限での自動クローズ
        if treasury.member_count >= 100 {
            treasury.treasury_status = TreasuryStatus::Full;
        }

        Ok(())
    }

    pub fn process_refund_request(ctx: Context<ProcessRefundRequest>) -> Result<()> {
        let treasury = &mut ctx.accounts.guild_treasury;
        let member_data = &mut ctx.accounts.member_data;
        let current_slot = Clock::get()?.slot;

        if member_data.last_payment_slot + treasury.refund_period_slots > current_slot {
            let refund_amount = treasury.monthly_fee;
            
            // 返金処理のループ
            let mut remaining_refund = refund_amount;
            while remaining_refund > 0 {
                let chunk = std::cmp::min(remaining_refund, 100_000 * 10u64.pow(6));
                
                transfer(
                    ctx.accounts.refund_ctx(),
                    chunk,
                )?;
                
                remaining_refund -= chunk;
            }

            member_data.payment_status = PaymentStatus::Refunded;
            treasury.total_refunded += refund_amount;
            
            if member_data.total_paid >= refund_amount {
                member_data.total_paid -= refund_amount;
            }
        }

        Ok(())
    }
}

impl<'info> ProcessFeeCollection<'info> {
    fn collect_fee_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.member_token_account.to_account_info(),
                to: self.guild_vault.to_account_info(),
                authority: self.member.to_account_info(),
            }
        )
    }
}

impl<'info> ProcessRefundRequest<'info> {
    fn refund_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.guild_vault.to_account_info(),
                to: self.member_token_account.to_account_info(),
                authority: self.guild_treasury.to_account_info(),
            }
        )
    }
}

#[derive(Accounts)]
pub struct InitGuildTreasury<'info> {
    #[account(mut)]
    pub guild_leader: Signer<'info>,
    
    #[account(
        init,
        payer = guild_leader,
        space = 8 + GuildTreasury::INIT_SPACE,
        seeds = [b"treasury", guild_leader.key().as_ref()],
        bump
    )]
    pub guild_treasury: Account<'info, GuildTreasury>,
    
    pub fee_mint: Account<'info, Mint>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ProcessFeeCollection<'info> {
    pub member: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"treasury", guild_treasury.guild_leader.as_ref()],
        bump
    )]
    pub guild_treasury: Account<'info, GuildTreasury>,
    
    #[account(
        init_if_needed,
        payer = member,
        space = 8 + MemberData::INIT_SPACE,
        seeds = [b"member", guild_treasury.key().as_ref(), member.key().as_ref()],
        bump
    )]
    pub member_data: Account<'info, MemberData>,
    
    #[account(mut)]
    pub member_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub guild_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessRefundRequest<'info> {
    pub member: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"treasury", guild_treasury.guild_leader.as_ref()],
        bump
    )]
    pub guild_treasury: Account<'info, GuildTreasury>,
    
    #[account(
        mut,
        seeds = [b"member", guild_treasury.key().as_ref(), member.key().as_ref()],
        bump
    )]
    pub member_data: Account<'info, MemberData>,
    
    #[account(mut)]
    pub member_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub guild_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct GuildTreasury {
    pub guild_leader: Pubkey,
    pub fee_token_mint: Pubkey,
    pub monthly_fee: u64,
    pub refund_period_slots: u64,
    pub total_collected: u64,
    pub total_refunded: u64,
    pub member_count: u32,
    pub treasury_status: TreasuryStatus,
}

#[account]
#[derive(InitSpace)]
pub struct MemberData {
    pub last_payment_slot: u64,
    pub total_paid: u64,
    pub payment_status: PaymentStatus,
    pub is_first_payment: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum TreasuryStatus {
    Open,
    Full,
    Closed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum PaymentStatus {
    Current,
    Overdue,
    Refunded,
}