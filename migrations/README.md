# Database Migrations

This directory contains SQL migration scripts for the Employee Directory application.

## Overview

These migrations set up the authentication and authorization system by:
1. Adding `employee_id` foreign key to `users` table
2. Seeding initial test users for each role

## Running Migrations

### Using PostgreSQL command line:

```bash
# Connect to your database
psql -U postgres -d employee_directory

# Run migrations in order
\i migrations/001_add_employee_id_to_users.sql
\i migrations/002_seed_initial_users.sql
```

### Using DBeaver or other GUI tools:

1. Open SQL editor
2. Load and execute `001_add_employee_id_to_users.sql`
3. Load and execute `002_seed_initial_users.sql`

## Test Users

After running the seed script, the following test accounts will be available:

| Email | Password | Role | Employee |
|-------|----------|------|----------|
| `admin@company.com` | `password123` | ROLE_ADMIN | None (System Admin) |
| `leslie@luv2code.com` | `password123` | ROLE_HR | Leslie Andrews |
| `emma@luv2code.com` | `password123` | ROLE_IT | Emma Baumgarten |
| `avani@luv2code.com` | `password123` | ROLE_EMPLOYEE | Avani Gupta |

## ⚠️ Security Notes

- **CHANGE THESE PASSWORDS IMMEDIATELY IN PRODUCTION!**
- The password hashes are bcrypt with cost factor 12
- All test accounts use the same password for development convenience only
- In production, use strong, unique passwords for each account

## Role Permissions (Planned)

- **ROLE_ADMIN**: Full system access
- **ROLE_HR**: Create/Read/Update/Delete employee
- **ROLE_IT**: Create user accounts, assign roles, manage access
- **ROLE_EMPLOYEE**: View own profile only

## Rollback

To rollback the migrations, uncomment and run the DOWN sections at the bottom of each migration file.

## Next Steps

After running these migrations:
1. Verify users were created: `SELECT * FROM users;`
2. Test login with one of the accounts above
3. Implement JWT-based authentication in the Rust backend