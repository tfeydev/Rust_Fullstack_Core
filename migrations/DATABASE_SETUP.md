# Complete Database Setup Guide - From Zero to Ready

This guide will help you set up the complete database for the Employee Directory application from scratch.

## Prerequisites

- PostgreSQL installed (version 12 or higher)
- Access to PostgreSQL command line (`psql`) or a GUI tool like DBeaver

## Step 1: Create the Database

```bash
# Connect to PostgreSQL as superuser
psql -U postgres

# Create the database
CREATE DATABASE employee_directory;

# Connect to the new database
\c employee_directory
```

Or using command line directly:
```bash
createdb -U postgres employee_directory
```

## Step 2: Create Tables

### 2.1 Create `app_role` Table

```sql
CREATE TABLE app_role (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);
```

### 2.2 Create `employee` Table

```sql
CREATE TABLE employee (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### 2.3 Create `users` Table

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role_id INTEGER NOT NULL,
    employee_id INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT fk_users_role FOREIGN KEY (role_id) 
        REFERENCES app_role(id) ON DELETE RESTRICT,
    CONSTRAINT fk_users_employee FOREIGN KEY (employee_id) 
        REFERENCES employee(id) ON DELETE SET NULL
);
```

### 2.4 Create Indexes

```sql
-- Index for faster user lookups by email
CREATE INDEX idx_users_email ON users(email);

-- Index for faster lookups by employee_id
CREATE INDEX idx_users_employee_id ON users(employee_id);

-- Index for faster lookups by role
CREATE INDEX idx_users_role_id ON users(role_id);
```

## Step 3: Insert Roles

```sql
INSERT INTO app_role (id, name) VALUES
    (1, 'ROLE_ADMIN'),
    (2, 'ROLE_MANAGER'),
    (3, 'ROLE_HR'),
    (4, 'ROLE_IT'),
    (5, 'ROLE_EMPLOYEE');
```

## Step 4: Insert Sample employee

```sql
INSERT INTO employee (first_name, last_name, email) VALUES
    ('Leslie', 'Andrews', 'leslie@luv2code.com'),
    ('Emma', 'Baumgarten', 'emma@luv2code.com'),
    ('Avani', 'Gupta', 'avani@luv2code.com'),
    ('Yuri', 'Petrov', 'yuri@luv2code.com'),
    ('Juan', 'Vega', 'juan@luv2code.com');
```

## Step 5: Insert Test Users

⚠️ **Default password for all accounts: `password123`**

```sql
-- 1. System Admin (no employee link)
INSERT INTO users (email, password_hash, role_id, employee_id, created_at)
VALUES (
    'admin@company.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS',
    1, -- ROLE_ADMIN
    NULL,
    NOW()
);

-- 2. HR User - Leslie Andrews
INSERT INTO users (email, password_hash, role_id, employee_id, created_at)
VALUES (
    'leslie@luv2code.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS',
    3, -- ROLE_HR
    1,
    NOW()
);

-- 3. IT User - Emma Baumgarten
INSERT INTO users (email, password_hash, role_id, employee_id, created_at)
VALUES (
    'emma@luv2code.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS',
    4, -- ROLE_IT
    2,
    NOW()
);

-- 4. Employee User - Avani Gupta
INSERT INTO users (email, password_hash, role_id, employee_id, created_at)
VALUES (
    'avani@luv2code.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS',
    5, -- ROLE_EMPLOYEE
    3,
    NOW()
);
```

## Step 6: Verify Setup

### 6.1 Check All Tables

```sql
-- List all tables
\dt

-- Should show:
-- app_role
-- employee
-- users
```

### 6.2 Verify Roles

```sql
SELECT * FROM app_role ORDER BY id;
```

Expected output:
```
 id |      name      
----+----------------
  1 | ROLE_ADMIN
  2 | ROLE_MANAGER
  3 | ROLE_HR
  4 | ROLE_IT
  5 | ROLE_EMPLOYEE
```

### 6.3 Verify employee

```sql
SELECT id, first_name, last_name, email FROM employee ORDER BY id;
```

### 6.4 Verify Users with Roles

```sql
SELECT 
    u.id,
    u.email,
    r.name as role,
    e.first_name || ' ' || e.last_name as employee_name
FROM users u
JOIN app_role r ON u.role_id = r.id
LEFT JOIN employee e ON u.employee_id = e.id
ORDER BY u.id;
```

Expected output:
```
 id |         email          |      role      | employee_name  
----+------------------------+----------------+----------------
  1 | admin@company.com      | ROLE_ADMIN     | 
  2 | leslie@luv2code.com    | ROLE_HR        | Leslie Andrews
  3 | emma@luv2code.com      | ROLE_IT        | Emma Baumgarten
  4 | avani@luv2code.com     | ROLE_EMPLOYEE  | Avani Gupta
```

## Step 7: Test User Accounts

| Email | Password | Role | Access |
|-------|----------|------|--------|
| `admin@company.com` | `password123` | ROLE_ADMIN | Full system access |
| `leslie@luv2code.com` | `password123` | ROLE_HR | Manage employee |
| `emma@luv2code.com` | `password123` | ROLE_IT | Manage user accounts |
| `avani@luv2code.com` | `password123` | ROLE_EMPLOYEE | View own profile |

## Complete Setup Script

If you want to run everything at once, create a file `setup_database.sql`:

```sql
-- Create database (run this separately as postgres user)
-- CREATE DATABASE employee_directory;

-- Then run this after connecting to employee_directory:

-- 1. Create Tables
CREATE TABLE app_role (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE employee (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role_id INTEGER NOT NULL,
    employee_id INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT fk_users_role FOREIGN KEY (role_id) 
        REFERENCES app_role(id) ON DELETE RESTRICT,
    CONSTRAINT fk_users_employee FOREIGN KEY (employee_id) 
        REFERENCES employee(id) ON DELETE SET NULL
);

-- 2. Create Indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_employee_id ON users(employee_id);
CREATE INDEX idx_users_role_id ON users(role_id);

-- 3. Insert Roles
INSERT INTO app_role (id, name) VALUES
    (1, 'ROLE_ADMIN'),
    (2, 'ROLE_MANAGER'),
    (3, 'ROLE_HR'),
    (4, 'ROLE_IT'),
    (5, 'ROLE_EMPLOYEE');

-- 4. Insert Sample employee
INSERT INTO employee (first_name, last_name, email) VALUES
    ('Leslie', 'Andrews', 'leslie@luv2code.com'),
    ('Emma', 'Baumgarten', 'emma@luv2code.com'),
    ('Avani', 'Gupta', 'avani@luv2code.com'),
    ('Yuri', 'Petrov', 'yuri@luv2code.com'),
    ('Juan', 'Vega', 'juan@luv2code.com');

-- 5. Insert Test Users (password: password123)
INSERT INTO users (email, password_hash, role_id, employee_id, created_at) VALUES
    ('admin@company.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS', 1, NULL, NOW()),
    ('leslie@luv2code.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS', 3, 1, NOW()),
    ('emma@luv2code.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS', 4, 2, NOW()),
    ('avani@luv2code.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS', 5, 3, NOW());
```

Run it with:
```bash
psql -U postgres -d employee_directory -f migrations/setup_database.sql
```

## Environment Variables

Create or update your `.env` file:

```env
DATABASE_URL=postgres://postgres:your_password@localhost/employee_directory
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
JWT_EXPIRATION=86400  # 24 hours in seconds
```

## Troubleshooting

### Error: "database already exists"
```sql
DROP DATABASE IF EXISTS employee_directory;
CREATE DATABASE employee_directory;
```

### Error: "relation already exists"
```sql
-- Drop tables in reverse order (because of foreign keys)
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS employee CASCADE;
DROP TABLE IF EXISTS app_role CASCADE;

-- Then re-run the setup script
```

### Check PostgreSQL is running
```bash
# Linux
sudo systemctl status postgresql

# macOS
brew services list

# Check connection
psql -U postgres -l
```

## Next Steps
```sql
CREATE OR REPLACE VIEW users_extended AS
SELECT 
    u.id AS user_id,
    u.email,
    r.name AS role_name,
    u.employee_id,
    CASE
        WHEN e.id IS NULL THEN NULL
        ELSE CONCAT(e.first_name, ' ', e.last_name)
    END AS employee_name
FROM users u
JOIN app_role r ON u.role_id = r.id
LEFT JOIN employee e ON u.employee_id = e.id
ORDER BY u.id;


SELECT * FROM users_extended;
```

After completing this setup:

1. ✅ Database is ready
2. ✅ Test users are created
3. ⏭️ Configure Rust backend with DATABASE_URL
4. ⏭️ Implement JWT authentication
5. ⏭️ Build login page

---


### 
```sql
-- Vollzugriff auf bestehende Tabellen und Views
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO thor;

-- Vollzugriff auf bestehende Sequenzen (Auto-Increment IDs)
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO thor;

-- Vollzugriff auf bestehende Funktionen
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public TO thor;

-- Zukünftige Tabellen & Views automatisch freigeben
ALTER DEFAULT PRIVILEGES IN SCHEMA public 
GRANT ALL PRIVILEGES ON TABLES TO thor;

-- Zukünftige Sequenzen automatisch freigeben
ALTER DEFAULT PRIVILEGES IN SCHEMA public 
GRANT ALL PRIVILEGES ON SEQUENCES TO thor;

-- Zukünftige Funktionen automatisch freigeben
ALTER DEFAULT PRIVILEGES IN SCHEMA public 
GRANT ALL PRIVILEGES ON FUNCTIONS TO thor;

-- Zugriff auf das public-Schema selbst (für CREATE VIEW etc.)
GRANT ALL PRIVILEGES ON SCHEMA public TO thor;

```



### Trigger
```sql
CREATE OR REPLACE FUNCTION sync_employee_email_to_users()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    UPDATE users
    SET email = NEW.email
    WHERE employee_id = NEW.id;

    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS trg_sync_employee_email ON employee;

CREATE TRIGGER trg_sync_employee_email
AFTER UPDATE OF email ON employee
FOR EACH ROW
WHEN (NEW.email IS DISTINCT FROM OLD.email)
EXECUTE FUNCTION sync_employee_email_to_users();

```


### Trigger keine doppelten emails
```sql
CREATE OR REPLACE FUNCTION prevent_duplicate_user_emails()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    -- Prüfen, ob es eine andere User-ID mit gleicher Email gibt
    IF EXISTS (
        SELECT 1 FROM users
        WHERE email = NEW.email
          AND id <> COALESCE(NEW.id, -1)
    ) THEN
        RAISE EXCEPTION 'Email "%" exists already for another user', NEW.email;
    END IF;

    RETURN NEW;
END;
$$;


DROP TRIGGER IF EXISTS trg_prevent_duplicate_user_emails
ON users;

CREATE TRIGGER trg_prevent_duplicate_user_emails
BEFORE INSERT OR UPDATE OF email ON users
FOR EACH ROW
EXECUTE FUNCTION prevent_duplicate_user_emails();
```

### another trigger
```sql
-- Employee -> Users (guarded)
CREATE OR REPLACE FUNCTION public.sync_employee_email_to_users()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    -- Prevent recursion if guard is set
    IF coalesce(current_setting('app.sync_guard', true), '0') = '1' THEN
        RETURN NEW;
    END IF;

    -- Set guard for this transaction
    PERFORM set_config('app.sync_guard', '1', true);

    -- Update linked user(s)
    UPDATE public.users
    SET email = NEW.email
    WHERE employee_id = NEW.id
      AND (email IS DISTINCT FROM NEW.email);

    RETURN NEW;
END;
$$;


DROP TRIGGER IF EXISTS trg_sync_employee_email ON public.employee;

CREATE TRIGGER trg_sync_employee_email
AFTER UPDATE OF email ON public.employee
FOR EACH ROW
WHEN (NEW.email IS DISTINCT FROM OLD.email)
EXECUTE FUNCTION public.sync_employee_email_to_users();


-- Users -> Employee (guarded)
CREATE OR REPLACE FUNCTION public.sync_user_email_to_employee()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    -- Prevent recursion if guard is set
    IF coalesce(current_setting('app.sync_guard', true), '0') = '1' THEN
        RETURN NEW;
    END IF;

    -- Only act if this user is linked to an employee
    IF NEW.employee_id IS NULL THEN
        RETURN NEW;
    END IF;

    -- Set guard for this transaction so employee trigger won't loop back
    PERFORM set_config('app.sync_guard', '1', true);

    -- Update employee email only if different
    UPDATE public.employee
    SET email = NEW.email
    WHERE id = NEW.employee_id
      AND (email IS DISTINCT FROM NEW.email);

    RETURN NEW;
END;
$$;


DROP TRIGGER IF EXISTS trg_sync_user_email ON public.users;

CREATE TRIGGER trg_sync_user_email
AFTER UPDATE OF email ON public.users
FOR EACH ROW
WHEN (NEW.email IS DISTINCT FROM OLD.email)
EXECUTE FUNCTION public.sync_user_email_to_employee();


CREATE OR REPLACE FUNCTION public.prevent_duplicate_user_emails()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.users
        WHERE email = NEW.email
          AND id <> COALESCE(NEW.id, -1)
    ) THEN
        RAISE EXCEPTION 'EMAIL_EXISTS';
    END IF;
    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS trg_prevent_duplicate_user_emails ON public.users;

CREATE TRIGGER trg_prevent_duplicate_user_emails
BEFORE INSERT OR UPDATE OF email ON public.users
FOR EACH ROW
EXECUTE FUNCTION public.prevent_duplicate_user_emails();


-- Testing
BEGIN;

-- check before
SELECT id, email FROM public.employee WHERE id = 1;
SELECT id, email, employee_id FROM public.users WHERE employee_id = 1;

-- update users (should propagate to employee)
UPDATE public.users SET email='sync-test-user@example.com' WHERE id = <user-id>;

-- check after within same transaction
SELECT id, email FROM public.employee WHERE id = <employee-id>;
SELECT id, email FROM public.users WHERE id = <user-id>;

ROLLBACK;

```

**📝 Note:** Remember to change all default passwords in production!