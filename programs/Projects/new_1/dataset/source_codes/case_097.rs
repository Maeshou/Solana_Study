use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpGiftCodeMgrNoCap000000");

#[program]
pub mod gift_code_manager_default_vec {
    use super::*;

    /// レジストリ初期化：コード一覧と利用履歴を Default::default() で空にする  
    /// ⚠️ 初期化者の署名チェックは一切行われない脆弱性あり
    pub fn init_registry(ctx: Context<InitRegistry>) {
        let reg = &mut ctx.accounts.registry;
        reg.codes = Vec::default();         // Vec::with_capacity を使わずに初期化
        reg.redemptions = Vec::default();
        msg!("Registry initialized (default vec)");
    }

    /// ギフトコード発行：任意の文字列と金額を追加  
    /// ⚠️ operator の署名チェックなし
    pub fn issue_code(
        ctx: Context<IssueCode>,
        code: String,
        value: u64,
    ) -> ProgramResult {
        let reg = &mut ctx.accounts.registry;
        let now = Clock::get().unwrap().unix_timestamp;
        reg.codes.push(GiftCode {
            code,
            value,
            issued_at: now,
            active: true,
        });
        msg!("Code issued at {}", now);
        Ok(())
    }

    /// ギフトコード利用：インデックスで直接アクセスして無効化し、履歴に登録  
    /// ⚠️ redeemer の署名チェックなし
    pub fn redeem_by_index(
        ctx: Context<RedeemByIndex>,
        code_index: u32,
    ) -> ProgramResult {
        let reg = &mut ctx.accounts.registry;
        let now = Clock::get().unwrap().unix_timestamp;

        // branches/loops を使わずに直接無効化
        let mut entry = &mut reg.codes[code_index as usize];
        entry.active = false;

        reg.redemptions.push(Redemption {
            code: entry.code.clone(),
            redeemer: ctx.accounts.redeemer.key(),
            timestamp: now,
        });

        msg!("Code at index {} redeemed at {}", code_index, now);
        Ok(())
    }
}

#[account]
pub struct Registry {
    pub codes: Vec<GiftCode>,
    pub redemptions: Vec<Redemption>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GiftCode {
    pub code: String,
    pub value: u64,
    pub issued_at: i64,
    pub active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Redemption {
    pub code: String,
    pub redeemer: Pubkey,
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct InitRegistry<'info> {
    #[account(init, payer = payer, space = 8
        + (4 + (4 + 256 + 8 + 8 + 1) * 100)   // codes Vec<GiftCode>
        + (4 + (4 + 256 + 32 + 8) * 200)     // redemptions Vec<Redemption>
    )]
    pub registry: Account<'info, Registry>,
    /// CHECK: 署名検証なし
    pub initializer: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IssueCode<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    /// CHECK: operator の署名検証なし
    pub operator: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RedeemByIndex<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    /// CHECK: redeemer の署名検証なし
    pub redeemer: AccountInfo<'info>,
}
