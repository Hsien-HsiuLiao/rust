#  Axum

https://www.shuttle.dev/blog/2024/01/31/write-a-rest-api-rust#adding-a-database--migrations-in-rust

```shuttle deploy```

https://shuttle-axum-5za5.shuttle.app/

https://shuttle-axum-5za5.shuttle.app/mock_driver_arrives

---

# Axum REST API

A Rust-based REST API built with Axum framework, featuring user management endpoints and mock driver simulation.

## Features

- User CRUD operations (Create, Read, Update, Delete)
- Mock driver arrival/departure simulation
- PostgreSQL database integration
- RESTful API design

## API Endpoints

- `GET /` - Hello world message
- `GET /mock_driver_arrives` - Simulate driver arrival
- `GET /mock_driver_leaves` - Simulate driver departure
- `GET /users` - Retrieve all users
- `POST /users` - Create a new user
- `GET /users/:id` - Retrieve user by ID
- `PUT /users/:id` - Update user by ID
- `DELETE /users/:id` - Delete user by ID

## Local Development

### Prerequisites

- Rust (latest stable version)
- PostgreSQL database
- Shuttle CLI

### Setup

1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. Install Shuttle CLI:
   ```bash
   cargo install shuttle
   ```

3. Clone and navigate to the project:
   ```bash
   cd shuttle-axum-restapi
   ```

4. Run locally:
   ```bash
   shuttle run
   ```

## Deployment Options

### Option 1: Deploy to Shuttle (Recommended for Rust)

Shuttle is the recommended platform for Rust applications as it provides native Rust support.

1. Login to Shuttle:
   ```bash
   shuttle login
   ```

2. Deploy:
   ```bash
   shuttle deploy
   ```

3. Your API will be available at: `https://shuttle-axum-5za5.shuttle.app/`

### Option 2: Deploy to Vercel

Vercel can be used for deployment, though it requires additional configuration for Rust applications.

#### Prerequisites

- Vercel CLI installed
- Vercel account

#### Setup

1. Install Vercel CLI:
   ```bash
   npm i -g vercel
   ```

2. Login to Vercel:
   ```bash
   vercel login
   ```

3. Deploy:
   ```bash
   vercel --prod
   ```

#### Vercel Configuration

The project includes:
- `vercel.json` - Vercel deployment configuration
- `.vercelignore` - Files to exclude from deployment

#### Important Notes for Vercel

- Vercel has limited Rust support compared to Shuttle
- The current configuration uses Node.js runtime for API functions
- Database connections may require additional configuration
- Consider using Shuttle for production Rust applications

## Database

The application uses PostgreSQL with the following schema:

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    age INTEGER NOT NULL
);
```

## Environment Variables

For local development, Shuttle automatically provides:
- Database connection string
- Other necessary environment variables

For Vercel deployment, you'll need to configure:
- Database connection strings
- Any other required environment variables

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test locally with `shuttle run`
5. Submit a pull request

## License

This project is open source and available under the MIT License.