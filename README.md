# ⚡ SolQuest: The Proof of Hustle Protocol

![Solana Banner](https://img.shields.io/badge/Network-Solana_Devnet-blueviolet?style=for-the-badge&logo=solana)
![Anchor Version](https://img.shields.io/badge/Anchor-v0.30.1-blue?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

**SolQuest** es un protocolo de recompensas descentralizado construido en Solana que transforma la productividad en activos digitales. A través de un modelo de **Program Derived Addresses (PDAs)**, permite a los usuarios completar desafíos y recibir micro-pagos instantáneos en SOL y puntos de reputación on-chain.

---

### 🚀 Características Principales

* **Micro-Rewards Instantáneos:** Liquidación de recompensas en tiempo real tras la validación de la tarea.
* **Reputación On-Chain:** Sistema de puntos (`UserStats`) que vive permanentemente en la blockchain.
* **Arquitectura Escalable:** Uso de PDAs para garantizar que cada usuario y tarea tengan su propio espacio de almacenamiento eficiente.
* **Seguridad Anchor:** Validaciones robustas para evitar dobles reclamos y accesos no autorizados.

---

### 🛠️ Stack Tecnológico

* **Smart Contract:** Rust + [Anchor Framework](https://www.anchor-lang.com/)
* **Frontend:** React + `@solana/web3.js` + `@solana/wallet-adapter`
* **Testing:** Mocha & TypeScript
* **Despliegue:** Solana Devnet

---

### 🏗️ Arquitectura de Cuentas (PDAs)

El protocolo utiliza una estructura de cuentas optimizada para minimizar el costo de **Rent**:

| Cuenta | Semillas (Seeds) | Propósito |
| :--- | :--- | :--- |
| **UserStats** | `["user-stats", user_pubkey]` | Guarda el progreso y puntos del usuario. |
| **Task** | `["task", admin_pubkey, task_id]` | Define la recompensa y el estado de la tarea. |

---

### 📁 Fragmentos de Código Core (Rust)

```rust
#[account]
pub struct Task {
    pub id: u64,
    pub reward: u64,
    pub is_completed: bool,
}

#[account]
pub struct UserStats {
    pub user: Pubkey,
    pub tasks_completed: u64,
    pub points: u64,
}

#[error_code]
pub enum TaskError {
    #[msg("Esta tarea ya ha sido completada.")]
    AlreadyCompleted,
}
