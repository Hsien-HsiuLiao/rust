# Hyper 1.7.0 REST Microservice - Vercel Deployment

This microservice demonstrates a full REST API using hyper 1.7.0 with user management capabilities, deployable to Vercel as a serverless function using the latest `vercel-rust@4.0.9` runtime.

## Features

- **hyper 1.7.0** - Modern HTTP library
- **Full REST API** - CRUD operations for user management
- **Route handling** - Different responses for different HTTP methods and paths
- **Vercel deployment** - Serverless function deployment with vercel-rust@4.0.9
- **User database** - In-memory user storage with Slab

## Local Development

```bash
cargo run
```

Visit `http://localhost:8080` to see the service in action.

## REST API Endpoints

### **GET /** - Home Page
Returns HTML page with API documentation.

### **GET /user/{id}** - Get User
Retrieves user data by ID.
```bash
curl http://localhost:8080/user/1
```

### **POST /user/** - Create User
Creates a new user and returns the user ID.
```bash
curl -X POST http://localhost:8080/user/
```

### **PUT /user/{id}** - Update User
Updates an existing user.
```bash
curl -X PUT http://localhost:8080/user/1
```

### **DELETE /user/{id}** - Delete User
Removes a user by ID.
```bash
curl -X DELETE http://localhost:8080/user/1
```

## Vercel Deployment

### Prerequisites

1. Install Vercel CLI:
```bash
npm i -g vercel
```

2. Login to Vercel:
```bash
vercel login
```

### Deploy

1. Deploy to Vercel:
```bash
vercel
```

2. For production deployment:
```bash
vercel --prod
```

### How It Works

- **`api/index.rs`** - Main serverless function with full REST API
- **`vercel.json`** - Vercel configuration using vercel-rust@4.0.9
- **`Cargo.toml`** - Rust dependencies and binary configuration
- **Routes** - All `/api/*` requests are handled by the Rust function

### API Endpoints on Vercel

- **`/api/`** - Returns HTML page with API documentation
- **`/api/user/{id}`** - Full CRUD operations for users
- **Any other path** - Returns appropriate HTTP status codes

## Architecture

This microservice uses:
- **hyper 1.7.0** for HTTP handling
- **vercel_runtime 1.x** for serverless compatibility
- **tokio** for async runtime
- **Slab** for efficient user ID management
- **Arc<Mutex<...>>** for thread-safe shared state

## Benefits of Vercel Deployment

- **Serverless** - No server management needed
- **Global CDN** - Fast response times worldwide
- **Auto-scaling** - Handles traffic spikes automatically
- **Easy deployment** - Git-based deployments
- **Cost-effective** - Pay only for what you use
- **REST API ready** - Full CRUD operations available
- **Native Rust support** - Using vercel-rust@4.0.9 runtime

## Testing the API

After deployment, test your endpoints:

```bash
# Get the home page
curl https://your-vercel-url.vercel.app/api/

# Create a user
curl -X POST https://your-vercel-url.vercel.app/api/user/

# Get user by ID (replace {id} with actual ID)
curl https://your-vercel-url.vercel.app/api/user/{id}
```

## Troubleshooting

If you encounter build issues:
1. Ensure you're using the latest `vercel-rust@4.0.9` runtime
2. Check that your `Cargo.toml` has the correct binary configuration
3. Verify that `vercel_runtime = "1"` is specified in dependencies 