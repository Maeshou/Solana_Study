use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln2222222222222222222222222222222");

#[program]
pub mod magical_forge {
    use super::*;

    // 武器を鍛造し、工房をクローズする
    pub fn forge_weapon(ctx: Context<ForgeWeapon>) -> Result<()> {
        // 鍛造成功のロジック
        msg!("Weapon forged successfully! Chamber is now closed.");
        Ok(())
    }
    
    // 新しい鍛造工房をセットアップする（脆弱な再生成処理）
    pub fn setup_new_forge(ctx: Context<SetupNewForge>, temperature: u16, magic_essence: u64) -> Result<()> {
        let forge_account = ctx.accounts.forge_chamber.to_account_info();
        let smith = ctx.accounts.blacksmith.to_account_info();
        let forge_size: u64 = 72;

        // 設備コストを計算
        let setup_cost = 1_000_000 + magic_essence * 5;
        
        // 資金提供
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(&smith.key(), &forge_account.key(), setup_cost),
            &[smith.clone(), forge_account.clone()],
        )?;

        // スペース確保と所有権割り当て
        anchor_lang::solana_program::program::invoke(
            &system_instruction::allocate(&forge_account.key(), forge_size),
            &[forge_account.clone()],
        )?;
        anchor_lang::solana_program::program::invoke(
            &system_instruction::assign(&forge_account.key(), &crate::id()),
            &[forge_account.clone()],
        )?;

        // アカウントデータを手動で初期化
        let mut data = forge_account.try_borrow_mut_data()?;
        for i in 8..16 {
            data[i] = (magic_essence as u8) & (i as u8);
        }
        data[16..18].copy_from_slice(&temperature.to_le_bytes());

        Ok(())
    }
}

#[account]
pub struct ForgeChamber {
    pub magic_essence: u64,
    pub max_temperature: u16,
    pub uses_remaining: u8,
}

#[derive(Accounts)]
pub struct SetupNewForge<'info> {
    #[account(mut)]
    pub forge_chamber: UncheckedAccount<'info>,
    #[account(mut)]
    pub blacksmith: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ForgeWeapon<'info> {
    #[account(mut, close = blacksmith)]
    pub forge_chamber: Account<'info, ForgeChamber>,
    #[account(mut)]
    pub blacksmith: Signer<'info>,
}