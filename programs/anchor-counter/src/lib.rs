use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo, Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;

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

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.user_ata.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info()
                },
                &[&[
                    "mint".as_bytes(),
                    &[ctx.bumps.mint]
                ]]
            ),
            10*10^9
        )?;

        msg!("Minted 10 tokens to introducer");

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

    pub fn initialize_token_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Token mint initialized");
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
    pub token_program: Program<'info, Token>,
    #[account(
        seeds = ["mint".as_bytes()],
        bump,
        mut
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    pub user_ata: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
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

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        mint::decimals = 9,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct IntroAccountState {
    pub introducer: Pubkey,    // 32
    pub name: String,       // 4 + len()
    pub message: String, // 4 + len()
}