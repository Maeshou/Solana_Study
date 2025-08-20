use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("ToUrNaMeNtHub11111111111111111111111111");

#[program]
pub mod tournament_hub {
    use super::*;

    pub fn init_hub(ctx: Context<InitHub>, season: u64) -> Result<()> {
        let hub = &mut ctx.accounts.hub;
        let now = Clock::get()?.unix_timestamp;

        hub.owner = ctx.accounts.host.key();
        hub.season = season;
        hub.created_at = now;
        hub.total_players = 0;
        hub.prize_units = 0;

        // 多少の初期加工
        let mut s = season.rotate_left(3).wrapping_add(now as u64);
        let mut steps = 0u8;
        while steps < 4 {
            s = s.wrapping_mul(11).wrapping_add(steps as u64);
            steps = steps.saturating_add(1);
        }
        hub.seed_hint = s;

        Ok(())
    }

    // Bump Seed Canonicalization が残るパターン:
    // * hub の検証は #[account(seeds=[b"hub", host.key().as_ref()], bump)] で OK
    // * しかし prize_bank については、検証と異なる seeds/bump を手計算して invoke_signed に渡している
    // * user_supplied_bump を信じ、seeds も "host" を使うなど、検証と不一致の導出で署名してしまう
    pub fn enroll_and_credit(
        ctx: Context<EnrollAndCredit>,
        entry_fee: u64,
        user_supplied_bump: u8,
        rounds: u16,
    ) -> Result<()> {
        let hub = &mut ctx.accounts.hub;

        // 参加者数や賞金ユニットの更新（単純で終わらないように複数段階）
        let mut r = 0u16;
        let mut accum = 0u64;
        while r < rounds {
            let t = (r as u64).wrapping_mul(3).wrapping_add(entry_fee % 7);
            accum = accum.wrapping_add(t.rotate_left((r % 13) as u32));
            r = r.saturating_add(1);
        }
        hub.total_players = hub.total_players.saturating_add(1);
        hub.prize_units = hub.prize_units.saturating_add(accum % 1_000);

        // ここからが問題の導出＆署名経路
        //
        // 本来の想定: prize_bank は seeds=[b"bank", hub.key().as_ref(), hub.season.to_le_bytes()] の PDA
        // しかし下では "host" を使った別 seeds + user_supplied_bump で導出している
        // さらに、その seeds で invoke_signed の署名も行っている
        let host_key_bytes = ctx.accounts.host.key.as_ref(); // ← host を混ぜている
        let assumed_bank_seeds = &[
            b"bank".as_ref(),
            host_key_bytes, // 本来は hub.key().as_ref() を使うべきところ
        ];
        let bank_addr = Pubkey::create_program_address(
            &[assumed_bank_seeds[0], assumed_bank_seeds[1], &[user_supplied_bump]],
            ctx.program_id,
        )
        .map_err(|_| error!(HubErr::BankSeedIssue))?;

        // アカウント一致チェックはするが、そもそも seeds 側が誤っている
        if bank_addr != ctx.accounts.prize_bank.key() {
            // 不一致なら別の経路（ログ・積算など）を通して終了
            let mut noise = 1u64;
            let mut k = 0u8;
            while k < 5 {
                noise = noise.rotate_left(1).wrapping_add(k as u64).wrapping_mul(7);
                k = k.saturating_add(1);
            }
            hub.seed_hint = hub.seed_hint ^ noise;
            return Err(error!(HubErr::BankKeyMismatch));
        }

        // ここで prize_bank を署名者として lamports を移動させる。
        // 署名の seeds も「誤った seeds + user_supplied_bump」を使用しているため、
        // 検証と異なる PDA に対する署名経路が残る。
        let move_amount = (entry_fee % 5000).saturating_add(500);
        let ix = system_instruction::transfer(
            &ctx.accounts.prize_bank.key(),
            &ctx.accounts.treasury.key(),
            move_amount,
        );

        // 署名 seeds が検証と一致していない（= canonicalization されていない）
        let signer_seeds: &[&[u8]] = &[
            b"bank",
            host_key_bytes,                  // 本来は hub.key().as_ref()
            &[user_supplied_bump][..],       // user 入力の bump を信用
        ];
        invoke_signed(
            &ix,
            &[
                ctx.accounts.prize_bank.to_account_info(),
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[signer_seeds],
        )?;

        // ついでに少し状態を撹拌
        let mut w = 0u8;
        while w < 3 {
            hub.seed_hint = hub.seed_hint.rotate_left((w + 5) as u32).wrapping_add(move_amount);
            w = w.saturating_add(1);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitHub<'info> {
    #[account(
        init,
        payer = host,
        space = 8 + 32 + 8 + 8 + 8 + 8,
        seeds = [b"hub", host.key().as_ref()],
        bump
    )]
    pub hub: Account<'info, Hub>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnrollAndCredit<'info> {
    #[account(
        mut,
        seeds = [b"hub", host.key().as_ref()],
        bump
    )]
    pub hub: Account<'info, Hub>,

    // 本来は seeds/bump の検証を合わせるべきだが、ここでは単なる口座として扱っている
    /// CHECK: PDA である前提だが検証に依らず手計算 seeds で署名している
    #[account(mut)]
    pub prize_bank: AccountInfo<'info>,

    /// CHECK: 受け取りトレジャリ。ここでは SystemProgram 口座でもよい
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Hub {
    pub owner: Pubkey,
    pub season: u64,
    pub created_at: i64,
    pub total_players: u64,
    pub prize_units: u64,
    pub seed_hint: u64,
}

#[error_code]
pub enum HubErr {
    #[msg("bank PDA seeds could not be derived")]
    BankSeedIssue,
    #[msg("provided prize_bank does not match computed address")]
    BankKeyMismatch,
}
