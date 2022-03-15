use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod blog {
    use super::*;
    pub fn init_blog(ctx:Context<InitBlog>)-> ProgramResult {

        // get the accounts
        let blog_account = &mut ctx.accounts.blog_account;
        let genesis_post_account = &mut ctx.accounts.genesis_post_account;
        let authority = &mut ctx.accounts.authority;
        
        //setting the blog state
        blog_account.authority = authority.key();

        Ok(())
    }
    pub fn signup_user (ctx : Context<SignupUser>, name:String, avatar : String)-> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;
        
        user_account.name = name;
        user_account.avatar = avatar;
        user_account.authority = authority.key();
        Ok(())
    }

    pub fn update_user (ctx : Context<UpdateUser>, name: String, avatar: String)-> ProgramResult {

        let user_account = &mut ctx.accounts.user_account;

        user_account.name = name;
        user_account.avatar = avatar;

        Ok(())

    }
}

#[derive(Accounts)]
pub struct InitBlog<'info> {
    #[account(init, payer = authority, space = 8+32+32)]
    pub blog_account: Account <'info, BlogState>,
    #[account(init, payer = authority, space = 8+32+32+32+32+8)]
    pub genesis_post_account : Account <'info, PostState>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(mut, has_one = authority)]
    pub user_account : Account<'info, UserState>,
    pub authority  : Signer<'info>,
}

// State Values	Data Types	Size (in bytes)
// authority	Pubkey	32
// name	String	40
// avatar	String	120
// Pubkey: Pubkey is always 32 bytes and String is variable in size so it depends on your use case.
// String: String is an array of chars and each char takes 4 bytes in rust.
// Account Discriminator: All accounts created with Anchor needs 8 bytes

#[derive(Accounts)]
pub struct SignupUser <'info>{
    #[account(init, payer = authority, space = 8+40+120+32)]
    pub user_account : Account<'info, UserState>,
    pub authority : Signer<'info>,
    pub system_program : Program<'info, System>
}

#[account]
pub struct UserState {
    pub name :String,
    pub avatar : String,
    pub authority : Pubkey,
}

#[account]
pub struct BlogState {
    pub current_post_key: Pubkey,
    pub authority: Pubkey,
}


