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

- Vercel account (free at [vercel.com](https://vercel.com))
- GitHub, GitLab, or Bitbucket repository (recommended)

#### Method 1: Deploy via Vercel Website (Recommended for beginners)

1. **Push your code to a Git repository:**
   ```bash
   git add .
   git commit -m "Add Vercel deployment config"
   git push origin main
   ```

2. **Go to [vercel.com](https://vercel.com) and sign in**

3. **Click "New Project"**

4. **Import your Git repository:**
   - Connect your GitHub/GitLab/Bitbucket account if not already connected
   - Select your `shuttle-axum-restapi` repository
   - Click "Import"

5. **Configure the project:**
   - **Framework Preset:** Select "Other" or "No Framework"
   - **Root Directory:** Leave as default (or specify if your project is in a subdirectory)
   - **Build Command:** `export HOME=/root && source $HOME/.cargo/env && cargo build --release`
   - **Output Directory:** `target/release`
   - **Install Command:** `export HOME=/root && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source $HOME/.cargo/env && rustup default stable`

6. **Environment Variables (if needed):**
   - Add any required environment variables like database URLs
   - Click "Add" for each variable

7. **Click "Deploy"**

8. **Wait for deployment to complete:**
   - Vercel will build and deploy your project
   - You'll get a unique URL (e.g., `https://your-project.vercel.app`)

**⚠️ Important Notes for Vercel Deployment:**
- The build process may take several minutes as Vercel needs to install Rust and compile your application
- Rust applications on Vercel have limitations - they run as serverless functions with execution time limits
- Database connections may not work as expected in the serverless environment
- For production Rust applications, Shuttle is still recommended over Vercel

#### Method 2: Deploy via Vercel CLI

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

**⚠️ Important:** The `.vercelignore` file is configured to exclude unnecessary files but **keeps `migrations.sql`** to ensure it's available during the build process.

#### Important Notes for Vercel

- Vercel has limited Rust support compared to Shuttle
- The current configuration uses Node.js runtime for API functions
- Database connections may require additional configuration
- Consider using Shuttle for production Rust applications
- **Website deployment is easier for beginners** and provides a visual interface for configuration

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