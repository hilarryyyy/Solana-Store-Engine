use anchor_lang::prelude::*;

declare_id!("4Cr2gPGWSgaswj2YoYZN7n5Px7DU1t5doSiL3C2J8yoh"); 

#[program]
pub mod solana_store {
    use super::*;

    // 1. Registrar un producto nuevo
    pub fn create_product(ctx: Context<CreateProduct>, name: String, price: u64, stock: u32) -> Result<()> {
        let product = &mut ctx.accounts.product;
        product.owner = *ctx.accounts.seller.key;
        product.name = name;
        product.price = price;
        product.stock = stock;
        
        msg!("Producto creado: {} con stock de {}", product.name, product.stock);
        Ok(())
    }

    // 2. Comprar un producto (Reduce stock y transfiere SOL)
    pub fn purchase_product(ctx: Context<PurchaseProduct>) -> Result<()> {
        let product = &mut ctx.accounts.product;

        // Verificamos que haya stock
        if product.stock == 0 {
            return err!(ErrorCode::OutOfStock);
        }

        // Transferencia de SOL del comprador al vendedor
        let cpi_context = Box::new(anchor_lang::system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.seller.to_account_info(),
        });
        
        anchor_lang::system_program::transfer(
            CpiContext::new(ctx.accounts.system_program.to_account_info(), *cpi_context),
            product.price,
        )?;

        // Reducimos el stock
        product.stock -= 1;
        
        msg!("Compra exitosa. Stock restante: {}", product.stock);
        Ok(())
    }
}

#[account]
pub struct Product {
    pub owner: Pubkey,   // 32 bytes
    pub name: String,    // 4 bytes prefijo + caracteres
    pub price: u64,      // 8 bytes (en lamports)
    pub stock: u32,      // 4 bytes
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateProduct<'info> {
    #[account(
        init, 
        payer = seller, 
        space = 8 + 32 + (4 + name.len()) + 8 + 4,
        seeds = [b"product", seller.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseProduct<'info> {
    #[account(mut)]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: Esta es la cuenta que recibe el dinero (el dueño del producto)
    #[account(mut, constraint = seller.key() == product.owner)]
    pub seller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("No quedan unidades disponibles de este producto.")]
    OutOfStock,
}
