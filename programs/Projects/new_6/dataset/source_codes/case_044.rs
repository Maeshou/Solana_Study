use anchor_lang::prelude::*;

declare_id!("Sh0p17171717171717171717171717171717171717");

#[program]
pub mod nft_shop {
    use super::*;

    pub fn init_store(ctx: Context<InitStore>) -> Result<()> {
        let s = &mut ctx.accounts.store;
        s.admin = ctx.accounts.deployer.key();
        s.items = vec![100, 200, 300];
        s.prices = vec![50, 75, 100];
        s.version = 1;
        s.history = vec![];
        Ok(())
    }

    pub fn act_update(ctx: Context<UpdateStore>, factor: u8) -> Result<()> {
        let s = &mut ctx.accounts.store;
        let actor = &ctx.accounts.any_role;

        for i in 0..s.items.len() {
            if i % 2 == 0 {
                s.items[i] = s.items[i].saturating_sub((i as u64 + 1) * (factor as u64));
                s.prices[i] = s.prices[i].saturating_add(factor as u64 * 2);
            } else {
                s.items[i] = s.items[i].saturating_add(factor as u64);
                s.prices[i] = s.prices[i].saturating_sub((i as u64 + 1) * 3);
            }

            if s.prices[i] > 500 {
                s.prices[i] = s.prices[i] % 400;
            }

            let log = format!("Updated item {}: qty={}, price={}", i, s.items[i], s.prices[i]);
            s.history.push(log);
        }

        if s.history.len() > 20 {
            s.history.drain(0..5); // 古いログを削除
        }

        s.version = s.version.wrapping_add(1);
        s.admin = actor.key(); // Type Cosplay脆弱性
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStore<'info> {
    #[account(init, payer = deployer, space = 8 + 32 + 128)]
    pub store: Account<'info, Store>,
    #[account(mut)]
    pub deployer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateStore<'info> {
    #[account(mut)]
    pub store: Account<'info, Store>,
    /// CHECK: admin / updater 区別なし
    pub any_role: AccountInfo<'info>,
}

#[account]
pub struct Store {
    pub admin: Pubkey,
    pub items: Vec<u64>,
    pub prices: Vec<u64>,
    pub version: u8,
    pub history: Vec<String>,
}
