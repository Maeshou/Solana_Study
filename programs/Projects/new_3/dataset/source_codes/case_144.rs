use anchor_lang::prelude::*;
declare_id!("Loyalty11111111111111111111111111111111111");

/// ロイヤリティプログラム情報
#[account]
pub struct LoyaltyProgram {
    pub owner:         Pubkey,   // プログラム管理者
    pub program_name:  String,   // プログラム名
    pub total_members: u64,      // 登録メンバー数
}

/// メンバーアカウント情報
#[account]
pub struct MemberAccount {
    pub user:     Pubkey,        // メンバーの公開鍵
    pub program:  Pubkey,        // 本来は LoyaltyProgram.key() と一致すべき
    pub points:   u64,           // 保有ポイント
}

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 32 + 8)]
    pub program:        Account<'info, LoyaltyProgram>,
    #[account(mut)]
    pub owner:          Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnrollMember<'info> {
    /// LoyaltyProgram.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub program:        Account<'info, LoyaltyProgram>,

    #[account(init, payer = user, space = 8 + 32 + 32 + 8)]
    pub member:         Account<'info, MemberAccount>,

    #[account(mut)]
    pub user:           Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AwardPoints<'info> {
    /// LoyaltyProgram.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub program:        Account<'info, LoyaltyProgram>,

    /// member.program == program.key() の検証がないため、任意のプログラムの member を指定可能
    #[account(mut)]
    pub member:         Account<'info, MemberAccount>,

    pub owner:          Signer<'info>,
}

#[program]
pub mod loyalty_vuln {
    use super::*;

    /// プログラムを初期化
    pub fn initialize_program(ctx: Context<InitializeProgram>, name: String) -> Result<()> {
        let prog = &mut ctx.accounts.program;
        prog.owner = ctx.accounts.owner.key();
        prog.program_name = name.clone();
        prog.total_members = 0;
        msg!("Initialized program '{}' by {}", name, prog.owner);
        Ok(())
    }

    /// メンバー登録
    pub fn enroll_member(ctx: Context<EnrollMember>) -> Result<()> {
        let prog = &mut ctx.accounts.program;
        let mem  = &mut ctx.accounts.member;

        // 脆弱性ポイント：
        // mem.program = prog.key(); と代入するだけで、
        // 本来はプログラムIDの一致を検証する必要があるが省略されている
        mem.user    = ctx.accounts.user.key();
        mem.program = prog.key();
        mem.points  = 100; // 初期ポイント

        prog.total_members = prog.total_members.checked_add(1).unwrap();
        msg!(
            "User {} enrolled in '{}', total members: {}",
            mem.user,
            prog.program_name,
            prog.total_members
        );
        Ok(())
    }

    /// ポイント付与
    pub fn award_points(ctx: Context<AwardPoints>, amount: u64) -> Result<()> {
        let prog = &ctx.accounts.program;
        let mem  = &mut ctx.accounts.member;

        // 必須のチェック例（未実装）：
        // require_keys_eq!(
        //     mem.program,
        //     prog.key(),
        //     LoyaltyError::ProgramMismatch
        // );
        // または
        // #[account(address = program.key())]
        // pub member: Account<'info, MemberAccount>,

        // 検証なしに任意の member.points を増加させられる
        mem.points = mem.points.checked_add(amount).unwrap();
        msg!(
            "{} points awarded to {} in program '{}'",
            amount,
            mem.user,
            prog.program_name
        );
        Ok(())
    }
}

#[error_code]
pub enum LoyaltyError {
    #[msg("MemberAccount が指定のプログラムに属していません")]
    ProgramMismatch,
}
