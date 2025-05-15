# Simple expense tracker

This project aims to be a _sort of_ a monorepo for a fullstack app

> [!IMPORTANT]
> This is still a work in progress.

# Tech Stack
- Actix Web
- Actix Files
- SQLx
- PostgreSQL
- Astro
- Turborepo

# Set up
1. Write your `.env` file, make sure to set up `DATABASE_URL` parameter.
2. Run database migrations
```bash
cargo install sqlx-cli --no-default-features --features postgre
cargo sqlx migrate run
```
3. Build the frontend
```bash
cd apps/client
npm run build
```
4. Run the server
```bash
# At project root
cargo run
```

# Project structure
```txt
tba
```

# Roadmap
- [x] Set up a proper dev server with proxy and cors
- [x] Set up a monorepo with turbo
- [x] Set up linter for server and client
- Hot reloading the backend
- Set up authentication via session cookies
- Set up docker for easy deploy
- tl;dr finish the whole app lol
