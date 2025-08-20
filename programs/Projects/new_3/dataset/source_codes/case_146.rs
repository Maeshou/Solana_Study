use anchor_lang::prelude::*;
declare_id!("DAOFun11111111111111111111111111111111111");

/// DAO 全体の情報
#[account]
pub struct Dao {
    pub admin:          Pubkey, // DAO 管理者
    pub total_projects: u64,
}

/// プロジェクトごとの情報
#[account]
pub struct Project {
    pub creator:        Pubkey, // プロジェクト作成者
    pub dao:            Pubkey, // 本来は Dao.key() と一致すべき
    pub project_name:   String,
    pub funds_received: u64,
}

/// 資金割り当て記録
#[account]
pub struct FundingRecord {
    pub contributor: Pubkey, // 出資者
    pub project:     Pubkey, // 本来は Project.key() と一致すべき
    pub amount:      u64,
}

#[derive(Accounts)]
pub struct InitializeDao<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8)]
    pub dao:            Account<'info, Dao>,
    #[account(mut)]
    pub admin:          Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterProject<'info> {
    /// Dao.admin == admin.key() は検証される
    #[account(mut, has_one = admin)]
    pub dao:            Account<'info, Dao>,

    #[account(init, payer = creator, space = 8 + 32 + 32 + 4 + 64 + 8)]
    pub project:        Account<'info, Project>,

    #[account(mut)]
    pub creator:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AllocateFunds<'info> {
    /// Dao.admin == admin.key() は検証される
    #[account(mut, has_one = admin)]
    pub dao:            Account<'info, Dao>,

    /// Project.dao == dao.key() の検証が **ない**
    #[account(mut)]
    pub project:        Account<'info, Project>,

    /// FundingRecord.project == project.key() の検証も **ない**
    #[account(init_if_needed, payer = contributor, space = 8 + 32 + 32 + 8)]
    pub funding:        Account<'info, FundingRecord>,

    #[account(mut)]
    pub contributor:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod dao_funding_vuln {
    use super::*;

    /// DAO を初期化
    pub fn initialize_dao(ctx: Context<InitializeDao>) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        dao.admin = ctx.accounts.admin.key();
        dao.total_projects = 0;
        msg!("DAO initialized by {}", dao.admin);
        Ok(())
    }

    /// 新しいプロジェクトを登録
    pub fn register_project(ctx: Context<RegisterProject>, name: String) -> Result<()> {
        let dao  = &mut ctx.accounts.dao;
        let proj = &mut ctx.accounts.project;
        proj.creator = ctx.accounts.creator.key();
        proj.dao     = dao.key();
        proj.project_name   = name.clone();
        proj.funds_received = 0;
        dao.total_projects = dao.total_projects.checked_add(1).unwrap();
        msg!("Project '{}' registered under DAO {}", name, dao.key());
        Ok(())
    }

    /// プロジェクトに資金を割り当て
    pub fn allocate_funds(ctx: Context<AllocateFunds>, amount: u64) -> Result<()> {
        let dao     = &mut ctx.accounts.dao;
        let proj    = &mut ctx.accounts.project;
        let record  = &mut ctx.accounts.funding;

        // ─── 脆弱性ポイント ───
        // 1) project.dao と dao.key() の整合性検証がないため、
        //    別の DAO に属するプロジェクトを指定されても通ってしまう。
        // 2) record.project と proj.key() の検証もないため、
        //    攻撃者が用意した FundingRecord を通して任意のプロジェクトへ資金を割り当てられる。

        record.contributor = ctx.accounts.contributor.key();
        record.project     = proj.key();
        record.amount      = amount;
        proj.funds_received = proj.funds_received.checked_add(amount).unwrap();

        msg!(
            "DAO {} allocated {} lamports to project {}",
            dao.admin,
            amount,
            project.key()
        );
        Ok(())
    }
}
