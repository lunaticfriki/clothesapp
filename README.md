# Clothes CRUD API

A REST API for managing clothes inventory built with Rust using hexagonal (ports and adapters) architecture.

## Features

- **Hexagonal Architecture**: Clean separation between domain, application, infrastructure, and API layers
- **Full CRUD Operations**: Complete Create, Read, Update, Delete for Pants, Shirts, and Outfits
- **In-Memory Storage**: Thread-safe in-memory repositories using `Arc<RwLock<HashMap>>`
- **Seed Data**: Pre-loaded with 20 pants, 20 shirts, and 5 outfits
- **RESTful API**: Standard HTTP methods and status codes
- **CORS Enabled**: Ready for frontend consumption

## Domain Model

### Entities

- **Price**: Value object with `price` (f64) and optional `discounted_price`
- **Pants**: id, name, price, color, reference
- **Shirt**: id, name, price, color, reference
- **Outfit**: id, optional shirt_id, optional pants_id, is_completed (automatically calculated)

> **Note**: The `is_completed` field is automatically determined based on whether the outfit has both a shirt AND pants. You cannot manually set this field - it's always calculated by the system.

## Quick Start

```bash
# Build the project
cargo build

# Run the server
cargo run

# Server starts on http://localhost:8080
```

## API Endpoints

### Pants
- `GET /pants` - List all pants
- `POST /pants` - Create new pants
- `GET /pants/:id` - Get pants by ID
- `PUT /pants/:id` - Update pants
- `DELETE /pants/:id` - Delete pants

### Shirts
- `GET /shirts` - List all shirts
- `POST /shirts` - Create new shirt
- `GET /shirts/:id` - Get shirt by ID
- `PUT /shirts/:id` - Update shirt
- `DELETE /shirts/:id` - Delete shirt

### Outfits
- `GET /outfits` - List all outfits
- `POST /outfits` - Create new outfit
- `GET /outfits/:id` - Get outfit by ID
- `PUT /outfits/:id` - Update outfit
- `DELETE /outfits/:id` - Delete outfit

## Example Usage

```bash
# Get all pants
curl http://localhost:8080/pants

# Get specific pants
curl http://localhost:8080/pants/{id}

# Create new pants
curl -X POST http://localhost:8080/pants \
  -H "Content-Type: application/json" \
  -d '{
    "id": "unique-id",
    "name": "Blue Jeans",
    "price": {
      "price": 79.99,
      "discounted_price": 59.99
    },
    "color": "Blue",
    "reference": "P-001"
  }'

# Update pants
curl -X PUT http://localhost:8080/pants/{id} \
  -H "Content-Type: application/json" \
  -d '{
    "id": "{id}",
    "name": "Updated Name",
    "price": {
      "price": 89.99,
      "discounted_price": null
    },
    "color": "Navy",
    "reference": "P-001"
  }'

# Delete pants
curl -X DELETE http://localhost:8080/pants/{id}
```

## Architecture

The project follows hexagonal architecture principles:

```
src/
├── domain/              # Domain layer (entities, repository traits)
│   ├── entities/       # Pure domain models
│   └── repositories/   # Repository port interfaces (traits)
├── application/        # Application layer (services)
├── infrastructure/     # Infrastructure layer (adapters)
│   ├── persistence/   # Repository implementations
│   └── seed.rs        # Seed data
└── api/               # API layer (HTTP handlers)
    ├── handlers/      # HTTP request handlers
    └── routes.rs      # Route configuration
```

### Layers

- **Domain**: Core business entities and repository interfaces (ports)
- **Application**: Services containing business logic
- **Infrastructure**: Concrete implementations of repositories (adapters)
- **API**: HTTP handlers and routing using axum

## Technology Stack

- **Web Framework**: [axum](https://github.com/tokio-rs/axum) 0.7
- **Async Runtime**: [tokio](https://tokio.rs/) 1.48
- **Serialization**: [serde](https://serde.rs/) 1.0
- **UUID**: [uuid](https://github.com/uuid-rs/uuid) 1.0
- **CORS**: [tower-http](https://github.com/tower-rs/tower-http) 0.5

## Development

The application uses in-memory storage, so all data is reset when the server restarts. This makes it perfect for:
- Frontend development and testing
- API prototyping
- Learning Rust and hexagonal architecture

## Extending the Application

The hexagonal architecture makes it easy to:
- Add new entities by creating domain models and repository traits
- Swap storage implementations (e.g., add PostgreSQL) by implementing the repository traits
- Add new API endpoints by creating handlers in the API layer
- Modify business logic in the service layer without affecting other layers

## License

This is a boilerplate/template project for learning and development purposes.
# clothesapp
