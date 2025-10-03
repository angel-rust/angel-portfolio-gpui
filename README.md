# TREZZA TERMINAL - Point of Sale (GPUI)

A modern, high-performance point of sale system built with Rust, GPUI, and PostgreSQL.

## 🚀 Features

- **Modern UI**: Built with GPUI for native performance and beautiful design
- **Full-Stack Rust**: Frontend (GPUI), Backend (Axum), and Shared types
- **Real-time Inventory**: Track stock levels with automatic reordering alerts
- **Authentication**: Secure JWT-based authentication for employees
- **Order Management**: Complete order lifecycle from cart to completion
- **Receipt Printing**: Generate professional receipts
- **Database Migrations**: Automated PostgreSQL schema management
- **RESTful API**: Clean API design with comprehensive endpoints

## 📦 Architecture

```
trezza-terminal/
├── frontend/          # GPUI-based desktop application
│   ├── src/
│   │   ├── main.rs    # Main UI and application logic
│   │   ├── api.rs     # API client for backend communication
│   │   ├── state.rs   # Application state management
│   │   └── receipt.rs # Receipt generation and printing
│   └── Cargo.toml
├── backend/           # Axum web server
│   ├── src/
│   │   ├── main.rs    # Server setup and routing
│   │   ├── auth.rs    # Authentication logic
│   │   ├── config.rs  # Configuration management
│   │   ├── db/        # Database models and queries
│   │   ├── routes/    # API route handlers
│   │   └── services/  # Business logic
│   ├── migrations/    # SQL migration files
│   └── Cargo.toml
└── shared/            # Shared types and utilities
    ├── src/
    │   ├── types.rs   # Common data structures
    │   ├── errors.rs  # Error types
    │   └── constants.rs
    └── Cargo.toml
```

## 🛠️ Setup

### Prerequisites

- Rust (latest stable)
- PostgreSQL 14+
- Git

### Database Setup

1. Install PostgreSQL and create a database:
```bash
createdb trezza_terminal
```

2. Copy the example environment file:
```bash
cd backend
cp .env.example .env
```

3. Edit `.env` and update the `DATABASE_URL` with your PostgreSQL credentials.

### Running the Application

#### Terminal 1 - Backend Server
```bash
cd backend
cargo run --bin trezza-terminal-server
```

The server will:
- Run migrations automatically
- Start on `http://127.0.0.1:3000`
- Seed the database with sample data

Default admin credentials:
- Username: `admin`
- Password: `admin123`

#### Terminal 2 - Frontend Application
```bash
cd frontend
cargo run --bin trezza-terminal
```

## 📡 API Endpoints

### Authentication
- `POST /api/auth/login` - User login

### Products
- `GET /api/products` - List all products
- `GET /api/products/:id` - Get product by ID
- `GET /api/products/search?q=query` - Search products

### Orders
- `POST /api/orders` - Create new order (requires auth)
- `GET /api/orders/:id` - Get order details
- `POST /api/orders/:id/complete` - Complete order (requires auth)
- `POST /api/orders/:id/cancel` - Cancel order (requires auth)

### Inventory
- `GET /api/inventory/:product_id` - Get inventory for product
- `GET /api/inventory/low-stock` - Get low stock items
- `POST /api/inventory/:product_id/restock` - Restock product

## 🧪 Testing

Run all tests:
```bash
# Shared library tests
cd shared && cargo test

# Backend tests
cd backend && cargo test

# Frontend tests (when available)
cd frontend && cargo test
```

## 🎨 UI Features

- **Product Catalog**: Grid view with category filtering
- **Shopping Cart**: Real-time cart updates with add/remove
- **Payment Processing**: Multiple payment methods
- **Receipt Generation**: Automatic receipt creation and printing
- **Error Handling**: User-friendly error messages
- **Loading States**: Visual feedback for async operations

## 🔒 Security

- JWT-based authentication
- Password hashing (production-ready with bcrypt/argon2)
- SQL injection protection via SQLx
- CORS configuration
- Input validation

## 📊 Database Schema

The system includes comprehensive tables for:
- Users (employees with role-based access)
- Products and Categories
- Inventory tracking
- Orders and Order Items
- Sessions
- Audit Logs

See `backend/migrations/` for the full schema.

## 🎯 Development Roadmap

- [ ] Add barcode scanning support
- [ ] Implement customer loyalty program
- [ ] Add sales analytics dashboard
- [ ] Multi-location support
- [ ] Mobile companion app
- [ ] Cloud backup and sync

## 📝 License

Copyright © 2025 TREZZA TERMINAL

## 🤝 Contributing

This is a private project. For questions or issues, contact the development team.

---

**Built with ❤️ using Rust, GPUI, Axum, and PostgreSQL**
