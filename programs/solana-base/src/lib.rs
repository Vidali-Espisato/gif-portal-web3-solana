use anchor_lang::prelude::*;

declare_id!("6gFQc36zmVwiFpWpGoPGTgEXk3bCBThL8rpsvtvS9wrK");
// Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_base {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = GifItem{
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };
		
        base_account.gif_list.push(item);        
        base_account.total_gifs += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>  {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GifItem {
    pub gif_link: String,
    pub user_address: Pubkey
}


#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<GifItem>
}
