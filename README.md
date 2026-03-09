# 🕯️ LUMEN — Decentralized Storefront on Solana

Un marketplace minimalista y completamente **on-chain** para gestionar productos digitales utilizando **Program Derived Addresses (PDAs)**.

---

# ✨ Descripción General

LUMEN es un smart contract desarrollado con Anchor sobre la blockchain de Solana.  
El proyecto demuestra cómo implementar un sistema **CRUD descentralizado** para administrar inventarios digitales directamente en la red.

Cada producto se almacena como una **PDA única**, lo que garantiza:

- Integridad de los datos
- Propiedad verificable
- Direcciones determinísticas
- Eficiencia en costos de almacenamiento

---

# 🧩 Características Principales

## 🛒 Registro de Productos

Los usuarios pueden registrar artículos con:

- Nombre (máximo 32 caracteres)
- Precio en lamports
- Estado de disponibilidad
- Propietario (wallet creador)

Cada producto se almacena en una PDA derivada de:

```rust
["item_v1", owner_pubkey, title]
```

---

## 🔄 Actualización Segura

Solo el propietario del producto puede modificar sus datos.

Las operaciones permitidas incluyen:

- Cambiar el precio
- Modificar la disponibilidad

El contrato valida automáticamente la propiedad mediante:

```rust
has_one = owner
```

---

## 🗑️ Eliminación con Recuperación de Renta

Cuando un producto se elimina:

- La cuenta se cierra
- La renta almacenada se devuelve al propietario
- Se libera almacenamiento en la red

Esto permite que el sistema sea **rent-neutral**.

---

# 🏛️ Arquitectura del Programa

## 📦 Modelo de Datos

```rust
StoreItem {
    owner: Pubkey,
    title: String,
    price: u64,
    is_available: bool,
    bump: u8
}
```

Este modelo representa un producto dentro del marketplace.

---

## 🧭 Contextos

El programa utiliza tres contextos principales:

```rust
AddItem
UpdateItem
DeleteItem
```

### AddItem
Inicializa una nueva PDA para el producto.

### UpdateItem
Permite modificar el precio y disponibilidad del producto.

### DeleteItem
Cierra la cuenta y devuelve la renta al propietario.

---

# 🧱 Diseño Basado en PDAs

Las Program Derived Addresses permiten:

- Direcciones determinísticas
- Seguridad sin claves privadas
- Eliminación de índices globales
- Escalabilidad

La PDA de cada producto se deriva de:

```rust
["item_v1", owner_pubkey, title]
```

---

# 🔐 Seguridad y Buenas Prácticas

El contrato implementa varias validaciones:

### Validación de Propietario

Solo el creador del producto puede modificarlo o eliminarlo.

### Validación de Entradas

```rust
title length: 1 - 32 characters
price > 0
```

### Gestión de Seeds

Las seeds garantizan integridad y evitan colisiones de direcciones.

### Cierre Seguro de Cuentas

```rust
close = owner
```

Esto devuelve automáticamente la renta al propietario.

---

# ⚡ Eficiencia y Costos

LUMEN no utiliza listas globales de productos.

Cada producto es independiente y vive en su propia cuenta PDA.

Ventajas:

- menor uso de almacenamiento
- menor costo computacional
- recuperación de renta al eliminar productos

---

# 🧪 Pruebas

El proyecto incluye pruebas automatizadas con:

```
TypeScript
Mocha
Chai
```

Las pruebas verifican:

- Creación de productos
- Persistencia de datos
- Actualización de precio
- Validación de permisos
- Eliminación y recuperación de renta

---

# 🚀 Cómo Ejecutarlo Localmente

## Requisitos

```
Solana CLI 1.18.x
Anchor 0.30.x
Node.js 20.x
Yarn
```

---

## Instalación

```bash
yarn install
```

---

## Compilar el programa

```bash
anchor build
```

---

## Ejecutar pruebas

```bash
anchor test
```

---

# 📂 Estructura del Repositorio

```
LUMEN/
├── programs/
│   └── lumen/
│       └── src/
│           └── lib.rs
├── tests/
│   └── anchor.test.ts
├── migrations/
│   └── deploy.ts
├── Anchor.toml
└── README.md
```

---

# 🎯 Objetivo del Proyecto

LUMEN demuestra cómo construir un smart contract limpio, seguro y escalable en Solana utilizando Anchor.

El proyecto sirve como:

- ejemplo educativo
- base para marketplaces descentralizados
- demostración de uso de PDAs en sistemas CRUD
