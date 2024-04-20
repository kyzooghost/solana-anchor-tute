use anchor_lang::prelude::*;

declare_id!("ADDeZSdphmJAuFUXq3eEAdfkZQzFu6Adkz9mpkRnypns");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn add_intro(
        ctx: Context<AddIntro>,
        name: String,
        message: String
    ) -> Result<()> {
        msg!("Name: {}", name);
        msg!("Message: {}", message);

        let intro_account = &mut ctx.accounts.intro;
        intro_account.introducer = ctx.accounts.initializer.key();
        intro_account.name = name;
        intro_account.message = message;

        msg!("Intro Account Created");
        Ok(())
    }

    pub fn update_intro(
        ctx: Context<UpdateIntro>,
        name: String,
        message: String
    ) -> Result<()> {
        msg!("update_intro invoked");
        msg!("New Name: {}", name);
        msg!("New Message: {}", message);

        let intro_account = &mut ctx.accounts.intro;

        msg!("Old Name: {}", intro_account.name);
        msg!("Old Message: {}", intro_account.message);

        intro_account.name = name;
        intro_account.message = message;

        msg!("Intro Updated");
        Ok(())
    }

    pub fn delete_intro(
        _ctx: Context<DeleteIntro>,
    ) -> Result<()> {
        msg!("delete_intro invoked");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String, message: String)]
pub struct AddIntro<'info> {
    #[account(
        init, 
        seeds = [initializer.key().as_ref()],
        bump,
        payer = initializer, 
        space = 8 + 32 + 4 + name.len() + 4 + message.len()
    )]
    pub intro: Account<'info, IntroAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String, message: String)]
pub struct UpdateIntro<'info> {
    #[account(
        mut,
        seeds = [initializer.key().as_ref()],
        bump,
        realloc = 8 + 32 + 4 + name.len() + 4 + message.len(),
        realloc::payer = initializer,
        realloc::zero = false
    )]
    pub intro: Account<'info, IntroAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteIntro<'info> {
    #[account(
        mut,
        seeds = [initializer.key().as_ref()],
        bump,
        close = initializer
    )]
    pub intro: Account<'info, IntroAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct IntroAccountState {
    pub introducer: Pubkey,    // 32
    pub name: String,       // 4 + len()
    pub message: String, // 4 + len()
}