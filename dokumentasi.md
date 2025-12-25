src/
├── domain/           # CORE: Business Rules & Entities (Pure Rust, No SQLx/Axum)
│   ├── models.rs     # Structs: Account, Money, Transaction
│   └── errors.rs     # Domain Errors: InsufficientFunds, InvalidAccount
├── ports/            # INTERFACES: Traits defining what the system needs
│   ├── repository.rs # Trait: AccountRepository (Save, Find, Update)
│   └── services.rs   # Trait: TransferService
├── adapters/         # INFRASTRUCTURE: Implementation details
│   ├── api/          # Inbound: Axum Handlers, DTOs
│   └── persistence/  # Outbound: SQLx implementation of repositories
├── services/         # APPLICATION LAYER: Orchestration (Use Cases)
│   └── transfer.rs   # Logic: "Debit A, Credit B" in one transaction
├── config.rs         # Environment variables
└── main.rs           # Dependency Injection & Entry point