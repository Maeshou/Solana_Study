use anchor_lang::prelude::*;

declare_id!("Var8Art8888888888888888888888888888888888");

#[program]
pub mod varied_article {
    use super::*;

    pub fn create(ctx: Context<Create>, title: String) -> Result<()> {
        let a = &mut ctx.accounts.article;
        a.title = title;
        a.len = a.title.len() as u32;
        Ok(())
    }

    pub fn analyze(ctx: Context<Analyze>, keywords: Vec<String>) -> Result<()> {
        let mut count = 0;
        for kw in keywords.iter() {
            // contains メソッドのみ利用
            if ctx.accounts.article.title.contains(kw) {
                count += 1;
            }
        }
        let ana = &mut ctx.accounts.analysis_account;
        ana.matches = count;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = author, space = 8 + 64 + 4)]
    pub article: Account<'info, ArticleData>,
    #[account(mut)] pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Analyze<'info> {
    pub article: Account<'info, ArticleData>,
    #[account(mut, init, payer = author, space = 8 + 4)]
    pub analysis_account: Account<'info, AnalysisData>,
    #[account(mut)] pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ArticleData {
    pub title: String,
    pub len: u32,
}

#[account]
pub struct AnalysisData {
    pub matches: u32,
}
