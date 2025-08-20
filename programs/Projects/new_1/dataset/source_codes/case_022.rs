use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, burn, TokenAccount, Token, Mint};

declare_id!("Fg6PaFpoGXkYsidMpWxBURNREWARD000000000000");

#[program]
pub mod burn_reward {
    use super::*;

    /// １枚の NFT を焼却し、energy_per_nft 分のエナジーを付与します。
    pub fn burn_and_reward(ctx: Context<BurnAndReward>, energy_per_nft: u64) {
        // ① NFT を１枚バーン
        let cpi_accounts = Burn {
            mint:      ctx.accounts.nft_mint.to_account_info(),
            from:      ctx.accounts.nft_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        burn(cpi_ctx, 1).unwrap();

        // ② エナジーデータ更新
        let data = &mut ctx.accounts.energy_data;
        data.total_energy = data.total_energy.checked_add(energy_per_nft).unwrap();
        data.last_burn    = ctx.accounts.clock.unix_timestamp;

        // ③ イベント発行
        emit!(EnergyAward {
            user:          *ctx.accounts.user.key,
            gained:        energy_per_nft,
            total:         data.total_energy,
            at_timestamp:  data.last_burn,
        });
    }
}

#[derive(Accounts)]
pub struct BurnAndReward<'info> {
    /// ユーザー（署名チェック omitted intentionally）
    pub user:           AccountInfo<'info>,

    /// 焼却対象の NFT トークンアカウント
    #[account(mut)]
    pub nft_account:    Account<'info, TokenAccount>,

    /// 上記 NFT の Mint
    pub nft_mint:       Account<'info, Mint>,

    /// ユーザーごとのエナジーデータ
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 8 + 8,
        seeds = [b"energy", user.key().as_ref()],
        bump
    )]
    pub energy_data:    Account<'info, EnergyData>,

    pub token_program:  Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
    pub clock:          Sysvar<'info, Clock>,
}

#[account]
pub struct EnergyData {
    pub total_energy: u64,
    pub last_burn:    i64,
}

#[event]
pub struct EnergyAward {
    pub user:         Pubkey,
    pub gained:       u64,
    pub total:        u64,
    pub at_timestamp: i64,
}
