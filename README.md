# 🛒 Solana Store Engine

Sistema de inventario descentralizado donde los productos son cuentas en la blockchain de Solana.

## 🛠 Instrucciones de Uso
1. **Build:** Haz clic en el martillo en Solana Playground.
2. **Deploy:** Ve a la pestaña de Deploy y confirma.
3. **Test:** - Usa `create_product` con un nombre (ej. "Laptop"), precio en lamports y stock inicial.
   - Usa `purchase_product` para simular una venta. El stock bajará automáticamente y el dinero se moverá entre cuentas.

## 🔍 Detalles Técnicos
- **PDAs:** Los productos se identifican de forma única mediante semillas (seeds) compuestas por el prefijo "product", la dirección del vendedor y el nombre del producto.
- **Seguridad:** El programa valida mediante `constraints` que solo el dueño del producto reciba el pago.
- **Gestión de Stock:** Si el stock llega a 0, la transacción de compra falla devolviendo un error personalizado.
