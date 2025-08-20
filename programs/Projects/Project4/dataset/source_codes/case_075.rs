use anchor_lang::prelude::*;

declare_id!("SafeEx21Encrypt11111111111111111111111111111");

#[program]
pub mod example21 {
    use super::*;

    pub fn init_encrypt(
        ctx: Context<InitEncrypt>,
        rounds: u8,
    ) -> Result<()> {
        let e = &mut ctx.accounts.encrypt;
        e.rounds         = rounds;
        e.key_strength   = 128;
        e.secure_flag    = false;

        // ラウンド数に応じて強度調整
        if rounds >= 10 {
            e.key_strength = 256;
            e.secure_flag  = true;
        }
        Ok(())
    }

    pub fn process_round(
        ctx: Context<ProcessRound>,
    ) -> Result<()> {
        let e = &mut ctx.accounts.encrypt;
        // 各ラウンドで強度を少し増加
        let mut i = 0u8;
        while i < e.rounds {
            e.key_strength = e.key_strength.saturating_add(1);
            i += 1;
        }
        // 強度が512超ならフラグ
        if e.key_strength > 512 {
            e.secure_flag = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEncrypt<'info> {
    #[account(init, payer = user, space = 8 + 1 + 2 + 1)]
    pub encrypt: Account<'info, EncryptionData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessRound<'info> {
    #[account(mut)] pub encrypt: Account<'info, EncryptionData>,
}

#[account]
pub struct EncryptionData {
    pub rounds:        u8,
    pub key_strength:  u16,
    pub secure_flag:   bool,
}
