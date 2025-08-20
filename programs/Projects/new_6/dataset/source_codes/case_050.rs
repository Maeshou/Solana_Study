use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

declare_id!("TyPeCoSpLaYxLong11111111111111111111111");

#[program]
pub mod arena_challenge_gate {
    use super::*;

    pub fn start_challenge(ctx: Context<StartChallenge>, challenge_id: u64, bonus_points: u16) -> Result<()> {
        // --- 所有者チェックだけ（役割の取り違え防止なし） ---
        if ctx.accounts.cfg.owner != crate::ID {
            return Err(ProgramError::IllegalOwner.into());
        }

        // --- デコード処理 ---
        let raw_data = ctx.accounts.cfg.data.borrow();
        let admin_data = ArenaAdminConfig::try_from_slice(&raw_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        // --- 複雑な条件判定（ネスト型） ---
        if admin_data.admin != ctx.accounts.signer.key() {
            if ctx.accounts.signer.key().to_bytes()[0] % 2 == 0 {
                // 偶数キーの署名者は特例チェック
                if admin_data.admin.to_bytes()[0] == 0 {
                    msg!("Special override: Even signer, zero admin byte");
                } else {
                    return Err(ProgramError::MissingRequiredSignature.into());
                }
            } else {
                return Err(ProgramError::MissingRequiredSignature.into());
            }
        }

        // --- 挑戦の進行 ---
        let mut total_points: u64 = 0;
        let mut multiplier: u64 = 1;
        let mut combo_count: u64 = 0;

        for i in 0..10 {
            let step_score = (challenge_id as u64).wrapping_add(i * 7) % 100;
            if step_score > 50 {
                total_points += step_score as u64;
                combo_count += 1;
                if combo_count > 3 {
                    multiplier += 1;
                }
            } else {
                // コンボ途切れ処理
                if combo_count > 0 {
                    total_points += combo_count * 5;
                }
                combo_count = 0;
            }

            // ボーナスの適用条件
            if bonus_points > 0 {
                if (i as u16) < bonus_points {
                    total_points += (bonus_points - i as u16) as u64;
                }
            }

            // 中間報告
            if i % 3 == 0 {
                msg!("Round {}: score={}, total={}, multiplier={}", i, step_score, total_points, multiplier);
            }
        }

        // --- 最終計算 ---
        total_points *= multiplier;
        if total_points > 500 {
            total_points = 500; // 上限
        }

        // --- 状態の書き換え ---
        let mut arena_state = ctx.accounts.arena_state.load_mut()?;
        arena_state.last_challenge_id = challenge_id;
        arena_state.last_score = total_points as u32;
        arena_state.last_admin_key = admin_data.admin;
        arena_state.last_bonus_awarded = bonus_points > 0;

        // --- 追加の長めの処理 ---
        let mut reward_tokens: u64 = 0;
        for j in 0..5 {
            let calc = total_points.wrapping_mul(j + 1);
            if calc % 4 == 0 {
                reward_tokens += calc / 4;
            } else {
                reward_tokens += calc % 4;
            }
            if reward_tokens > 1000 {
                reward_tokens = 1000;
            }
        }

        arena_state.last_reward_tokens = reward_tokens;

        msg!("Challenge completed: id={}, score={}, rewards={}", challenge_id, total_points, reward_tokens);
        Ok(())
    }

    pub fn write_player_profile(ctx: Context<WriteProfile>, key: Pubkey) -> Result<()> {
        // PlayerProfile と ArenaAdminConfig のレイアウトは同じ（Pubkey1つ）
        let profile = PlayerProfile { player: key };
        let bytes = profile.try_to_vec().map_err(|_| ProgramError::InvalidInstructionData)?;
        ctx.accounts.cfg.data.borrow_mut()[..32].copy_from_slice(&bytes);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartChallenge<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>, // ← 脆弱ポイント
    pub signer: Signer<'info>,
    #[account(zero_copy)]
    pub arena_state: AccountLoader<'info, ArenaState>,
}

#[derive(Accounts)]
pub struct WriteProfile<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>, // ← ディスクリミネータ検証なしで上書き可能
}

#[account(zero_copy)]
pub struct ArenaState {
    pub last_challenge_id: u64,
    pub last_score: u32,
    pub last_admin_key: Pubkey,
    pub last_bonus_awarded: bool,
    pub last_reward_tokens: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ArenaAdminConfig {
    pub admin: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PlayerProfile {
    pub player: Pubkey,
}
