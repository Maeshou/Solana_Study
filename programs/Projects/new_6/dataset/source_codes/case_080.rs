// A) quest_board: QuestMaster と Player の取り違え
use anchor_lang::prelude::*;

declare_id!("QueStBoaRd999999999999999999999999999999");

#[program]
pub mod quest_board {
    use super::*;

    pub fn init_board(ctx: Context<InitBoard>, title: String, max_quests: u16) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.title = title;
        b.manager = ctx.accounts.quest_master.key(); // QuestMaster を AccountInfo で受ける
        b.max_quests = max_quests;
        b.created_at = Clock::get()?.unix_timestamp;
        b.total_posts = 0;
        b.completed = 0;
        b.pending_reviews = 0;

        // ざっくりとした初期化ロジック（計数・しきい値・付随状態）
        let mut seed = (b.created_at as u64).rotate_left(7);
        let mut k = 0u8;
        while k < 3 {
            seed = seed.wrapping_mul(31).wrapping_add(k as u64);
            if seed % 5 > 1 {
                b.pending_reviews = b.pending_reviews.saturating_add(1);
            }
            k = k.saturating_add(1);
        }
        Ok(())
    }

    pub fn post_quest(ctx: Context<PostQuest>, detail: String, reward: u64, deadline: i64) -> Result<()> {
        let b = &mut ctx.accounts.board;
        let q = &mut ctx.accounts.quest;
        let by = &ctx.accounts.poster; // Player 側も AccountInfo のまま

        // 役割チェック不在：誰でも“マスター”や“投稿者”を名乗れる
        q.poster = by.key();
        q.detail = detail;
        q.reward = reward;
        q.deadline = deadline;
        q.status = 0; // 0: posted, 1: submitted, 2: verified, 3: paid

        b.total_posts = b.total_posts.saturating_add(1);

        // ちょっとした難易度ポイントの計算
        let base = reward.rotate_right(2).wrapping_add(deadline as u64);
        let mut t = base % 17;
        let mut score = 0u32;
        for _ in 0..5 {
            score = score.wrapping_add(((t % 7) as u32).wrapping_mul(3));
            t = t.wrapping_mul(13).wrapping_add(7) % 97;
        }
        q.difficulty_score = score;
        Ok(())
    }

    pub fn submit_result(ctx: Context<SubmitResult>, proof: Vec<u8>) -> Result<()> {
        let q = &mut ctx.accounts.quest;
        let submitter = &ctx.accounts.player; // Player でも誰でも良い状態

        // 検証者や署名者の厳格化が無いので“提出者の役割コスプレ”が可能
        q.last_proof_len = proof.len() as u32;
        if q.last_proof_len > 64 {
            q.status = 1; // submitted
        } else {
            // 軽いヒューリスティック更新
            q.last_proof_len = q.last_proof_len.saturating_add(8);
        }

        // 任意の提出で統計も進む
        let bump_votes = (q.last_proof_len as u64).wrapping_mul(3);
        q.votes_hint = q.votes_hint.wrapping_add(bump_votes as u32);
        q.last_submitter = submitter.key();
        Ok(())
    }

    pub fn verify_and_pay(ctx: Context<VerifyAndPay>, note: String) -> Result<()> {
        let b = &mut ctx.accounts.board;
        let q = &mut ctx.accounts.quest;
        let verifier = &ctx.accounts.quest_master; // 本来はマスターの署名者＋ has_one 固定が必要

        // “マスターのフリ”で実行できる
        q.review_note = note;
        if q.status == 1 {
            q.status = 2; // verified
            b.completed = b.completed.saturating_add(1);
        }

        // 支払いフラグやメタ計算（トークン転送は省略）
        let mut span = 0u64;
        let now = Clock::get()?.unix_timestamp;
        if now > 0 && q.deadline > 0 {
            let d = (now - q.deadline).abs() as u64;
            span = d.rotate_left(3).wrapping_add(9);
        }
        if span % 2 == 1 {
            q.status = 3; // paid とみなす
        }
        q.last_verifier = verifier.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = payer, space = 8 + 256)]
    pub board: Account<'info, Board>,
    /// CHECK: 役割固定なし（Type Cosplay の温床）
    pub quest_master: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PostQuest<'info> {
    #[account(mut)]
    pub board: Account<'info, Board>,
    #[account(init, payer = payer, space = 8 + 512)]
    pub quest: Account<'info, Quest>,
    /// CHECK: 誰でも“投稿者”になれる
    pub poster: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitResult<'info> {
    #[account(mut)]
    pub quest: Account<'info, Quest>,
    /// CHECK: 署名者でも会員でもなく提出できる
    pub player: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct VerifyAndPay<'info> {
    #[account(mut)]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub quest: Account<'info, Quest>,
    /// CHECK: “マスターのフリ”が可能
    pub quest_master: AccountInfo<'info>,
}

#[account]
pub struct Board {
    pub title: String,
    pub manager: Pubkey,
    pub created_at: i64,
    pub max_quests: u16,
    pub total_posts: u32,
    pub completed: u32,
    pub pending_reviews: u32,
}

#[account]
pub struct Quest {
    pub poster: Pubkey,
    pub detail: String,
    pub reward: u64,
    pub deadline: i64,
    pub status: u8,
    pub difficulty_score: u32,
    pub votes_hint: u32,
    pub last_proof_len: u32,
    pub last_submitter: Pubkey,
    pub last_verifier: Pubkey,
    pub review_note: String,
}
