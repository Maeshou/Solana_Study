use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpRaffleSysZZZZZZZZZZZZZZZZ");

#[program]
pub mod raffle_system {
    use super::*;

    /// ラッフルの設定：抽選当選者数と賞品数量を記録  
    /// ⚠️ 設定者の署名チェックは一切行われない脆弱性あり
    pub fn setup_raffle(
        ctx: Context<SetupRaffle>,
        winner_count: u32,
        prize_amount: u64,
    ) -> ProgramResult {
        let config = &mut ctx.accounts.config;
        config.winner_count = winner_count;
        config.prize_amount = prize_amount;
        Ok(())
    }

    /// 当選者を登録：当選者リストに追加  
    /// ⚠️ `operator` と `raffle_account` の署名チェックがなく、誰でも実行可能
    pub fn register_winner(
        ctx: Context<RegisterWinner>,
        winner: Pubkey,
    ) -> ProgramResult {
        let config = &mut ctx.accounts.config;
        config.winners.push(winner);
        msg!("Winner registered: {}", winner);
        Ok(())
    }

    /// 賞金請求：賞金を直接 vault から winner_account へ lamports で移動  
    /// CPI は使わず、AccountInfo のみで直接操作。署名検証もなし
    pub fn claim_prize(ctx: Context<ClaimPrize>) -> ProgramResult {
        let config = &ctx.accounts.config;
        let vault = ctx.accounts.vault.to_account_info();
        let recipient = ctx.accounts.winner_account.to_account_info();

        // lamports を直接移動（残高チェックなし）
        let amount = config.prize_amount;
        **vault.lamports.borrow_mut() -= amount;
        **recipient.lamports.borrow_mut() += amount;

        msg!("Prize of {} lamports claimed by {}", amount, recipient.key());
        Ok(())
    }
}

#[account]
pub struct RaffleConfig {
    pub winner_count: u32,
    pub prize_amount: u64,
    pub winners: Vec<Pubkey>,
}

#[derive(Accounts)]
pub struct SetupRaffle<'info> {
    #[account(init, payer = payer, space = 8 + 4 + 8 + 4 + 32 * 100)]
    pub config: Account<'info, RaffleConfig>,
    /// CHECK: 設定者の署名検証なし
    pub operator: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterWinner<'info> {
    #[account(mut)]
    pub config: Account<'info, RaffleConfig>,
    /// CHECK: raffle アカウントの署名検証なし
    pub raffle_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub config: Account<'info, RaffleConfig>,
    /// CHECK: 当選者アカウントの署名検証なし
    pub winner_account: AccountInfo<'info>,
    /// lamports 保管用 vault（署名検証なし）
    pub vault: AccountInfo<'info>,
}
