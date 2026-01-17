# AGENTS.md

Purpose
- This file guides coding agents working on this repo.
- Keep it short and up to date as the project evolves.

Project summary
- Farmtasker is a Rust web app (Leptos + Axum) being rebuilt from an unfinished prototype.
- Primary goal: a maintainable marketplace with a real database and workable tooling.
- Stripe is for managing oder payments only, not for product catalog data.

Current goals (v1)
- Refactor the code cleanly and cut prototyping stuff
- Product catalog stored in the local database (name, description, price, images, category, availability).
- Develop tooling to interact with the database cleanly inside the website itself (maybe inside admin panel)
- Products displayed + detail pages info read from the database.
- Cart + order creation stored in the database, but cart items may be stored locally for performance reasons.
- Customer records (email, address, phone) with order history in database.
- Delivery rules: Hobart local delivery and Saturday market pickup (details TBD later).
- Basic admin CRUD for products, orders, and customers.

Non-goals for now
- Refactor the design and pages front-end to suit the new back-end code.
- Managment tooling, e.g. admin panel.

Working agreements
- Prefer small, reviewable changes that land in working state, expect user confirmed manual testing every time. (ask user to confirm if code works as expected)
- Avoid runtime panics in server code (no unwrap/expect in non-test paths).
- When changing data models, add a migration and update seed data.
- Try think about performance concerns and whiether adding heavy dependencies or changing core stack is actually needed.
- Make the changes to the project managable, not implement unneeded features, current thought process is to refactor the project into managable state.
- Keep SSR/CSR parity in Leptos; use server functions for DB access.

Open decisions
- Database choice (SQLite vs Postgres).
- DB library/tooling (SQLx, SeaORM, Diesel, etc.).
- Exact delivery rules and pricing.

Current tasks (update as we go)
- [ ] Refactor the code into more managable state, cutting stuff that's unneeded
- [ ] Decide DB + ORM/tooling.
- [ ] Define schema for products, customers, orders, order_items, delivery.
- [ ] Add migrations and seed data.
- [ ] Build product list + detail views from DB.
- [ ] Implement cart -> order flow and order status tracking.
- [ ] Add admin CRUD screens and auth.
