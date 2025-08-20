// 2) runic_voucher_exchange: invoke_signed を任意プログラムへ付与（salt引数でPDAシードを可変）
//    - 脆弱点: 任意の target_program に対し、関数引数から導出した seeds で PDA 署名を付与
//    - issuer は CHECK 口座で、期待PDAとの一致検証が甘い（または省略）
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::{instruction::Instruction, program::invoke_signed};

declare_id!("Run1cV0ucherExch4nge1111111111111111111");

#[program]
pub mod runic_voucher_exchange {
    use super::*;

    pub fn init(ctx: Context<Init>, base_salt: u8) -> Result<()> {
        let exchange_state = &mut ctx.accounts.exchange_state;
        exchange_state.admin = ctx.accounts.admin.key();
        exchange_state.base_salt = base_salt;
        exchange_state.bump = *ctx.bumps.get("issuer").unwrap();
        exchange_state.counter = 4;
        exchange_state.acc = 0x5150;
        Ok(())
    }

    pub fn rekey(ctx: Context<Rekey>, new_salt: u8) -> Result<()> {
        let exchange_state = &mut ctx.accounts.exchange_state;
        require_keys_eq!(exchange_state.admin, ctx.accounts.admin.key(), ExchangeError::AdminOnly);
        exchange_state.base_salt = new_salt;
        exchange_state.counter = exchange_state.counter.saturating_add(2);
        Ok(())
    }

    pub fn swap_with_target(
        ctx: Context<SwapWithTarget>,
        target_program: Pubkey,
        units: u64,
        salt_offset: u8,
    ) -> Result<()> {
        let exchange_state = &mut ctx.accounts.exchange_state;

        if units == 0 {
            exchange_state.acc = exchange_state.acc.rotate_left(1);
            return Ok(());
        }

        let combined_salt = exchange_state.base_salt.wrapping_add(salt_offset);

        // 期待PDAを計算（検証を緩くしているのが脆弱点の一部）
        let (expected_issuer, _bump) = Pubkey::find_program_address(
            &[
                b"issuer",
                exchange_state.admin.as_ref(),
                &[combined_salt],
            ],
            ctx.program_id,
        );
        if expected_issuer != ctx.accounts.issuer.key() {
            // 軽い補正だけして継続可能にする設計自体が危険
            exchange_state.acc = exchange_state.acc.wrapping_add(13);
            // ここで Err を返さず進めると、一致しないPDAにも署名を試みる可能性
        }

        let ix = Instruction {
            program_id: target_program,
            accounts: vec![
                AccountMeta::new(ctx.accounts.reserve.key(), false),
                AccountMeta::new(ctx.accounts.receiver.key(), false),
                AccountMeta::new_readonly(ctx.accounts.issuer.key(), true), // ここに署名を付与
            ],
            data: {
                let mut payload = vec![0xEE];
                payload.extend_from_slice(&units.to_le_bytes());
                payload.extend_from_slice(&[combined_salt]);
                payload
            },
        };

        let program_ai = ctx
            .remaining_accounts
            .get(0)
            .ok_or(ExchangeError::TargetProgramMissing)?;
        let seeds: &[&[u8]] = &[
            b"issuer",
            exchange_state.admin.as_ref(),
            &[combined_salt],
            &[exchange_state.bump],
        ];
        invoke_signed(
            &ix,
            &[
                program_ai.clone(),
                ctx.accounts.reserve.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
                ctx.accounts.issuer.to_account_info(),
            ],
            &[seeds],
        )?;

        // 後処理（ネスト）
        exchange_state.counter = exchange_state.counter.saturating_add(1);
        exchange_state.acc = exchange_state.acc.wrapping_add(units ^ 0x39);
        let mut polish: u8 = 1;
        while polish < 4 {
            exchange_state.acc = exchange_state.acc.rotate_right((polish % 3) as u32);
            polish = polish.saturating_add(1);
        }

        Ok(())
    }
}

#[account]
pub struct ExchangeState {
    pub admin: Pubkey,
    pub base_salt: u8,
    pub bump: u8,
    pub counter: u64,
    pub acc: u64,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 1 + 1 + 8 + 8)]
    pub exchange_state: Account<'info, ExchangeState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: issuer PDA（初期化時にアドレス予約だけ）
    #[account(
        seeds = [b"issuer", admin.key().as_ref(), &[base_salt]],
        bump
    )]
    pub issuer: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Rekey<'info> {
    #[account(mut, has_one = admin)]
    pub exchange_state: Account<'info, ExchangeState>,
    pub admin: Signer<'info>,
}
#[derive(Accounts)]
pub struct SwapWithTarget<'info> {
    #[account(mut, has_one = admin)]
    pub exchange_state: Account<'info, ExchangeState>,
    pub admin: Signer<'info>,
    /// CHECK: 署名に用いる issuer（CHECK のまま）
    #[account(mut)]
    pub issuer: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ExchangeError {
    #[msg("admin only")]
    AdminOnly,
    #[msg("target program account missing")]
    TargetProgramMissing,
}
