-- ============================================================================
-- Complete Database Setup Script for Employee Directory
-- ============================================================================
-- Description: This script creates all tables, indexes, roles, and test data
-- Usage: psql -U postgres -d employee_directory -f migrations/setup_database.sql
--
-- IMPORTANT: Run this AFTER creating the database:
--   CREATE DATABASE employee_directory;
--   \c employee_directory
-- ============================================================================

-- ============================================================================
-- STEP 1: Create Tables
-- ============================================================================

-- Table: app_role
-- Stores role definitions for authorization
CREATE TABLE IF NOT EXISTS app_role (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

-- Table: employee
-- Stores employee master data
CREATE TABLE IF NOT EXISTS employee (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Table: users
-- Stores user authentication and authorization data
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role_id INTEGER NOT NULL,
    employee_id INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- Foreign key to app_role table
    CONSTRAINT fk_users_role
        FOREIGN KEY (role_id)
        REFERENCES app_role(id)
        ON DELETE RESTRICT,

    -- Foreign key to employee table
    CONSTRAINT fk_users_employee
        FOREIGN KEY (employee_id)
        REFERENCES employee(id)
        ON DELETE SET NULL
);

-- ============================================================================
-- STEP 2: Create Indexes for Performance
-- ============================================================================

-- Index for faster user lookups by email (login)
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Index for faster lookups by employee_id
CREATE INDEX IF NOT EXISTS idx_users_employee_id ON users(employee_id);

-- Index for faster role-based queries
CREATE INDEX IF NOT EXISTS idx_users_role_id ON users(role_id);

-- Index for employee email lookups
CREATE INDEX IF NOT EXISTS idx_employee_email ON employee(email);

-- ============================================================================
-- STEP 3: Insert Application Roles
-- ============================================================================

INSERT INTO app_role (id, name) VALUES
    (1, 'ROLE_ADMIN'),
    (2, 'ROLE_MANAGER'),
    (3, 'ROLE_HR'),
    (4, 'ROLE_IT'),
    (5, 'ROLE_EMPLOYEE')
ON CONFLICT (name) DO NOTHING;

-- Reset sequence to ensure proper ID generation
SELECT setval('app_role_id_seq', (SELECT MAX(id) FROM app_role));

-- ============================================================================
-- STEP 4: Insert Sample employee
-- ============================================================================

INSERT INTO employee (first_name, last_name, email) VALUES
    ('Leslie', 'Andrews', 'leslie@luv2code.com'),
    ('Emma', 'Baumgarten', 'emma@luv2code.com'),
    ('Avani', 'Gupta', 'avani@luv2code.com'),
    ('Yuri', 'Petrov', 'yuri@luv2code.com'),
    ('Juan', 'Vega', 'juan@luv2code.com')
ON CONFLICT (email) DO NOTHING;

-- Reset sequence to ensure proper ID generation
SELECT setval('employee_id_seq', (SELECT MAX(id) FROM employee));

-- ============================================================================
-- STEP 5: Insert Test Users
-- ============================================================================
-- Password for all test accounts: "password123"
-- Hash generated with: bcrypt::hash("password123", 12)
--
-- ⚠️  WARNING: CHANGE THESE PASSWORDS IN PRODUCTION!
-- ============================================================================

-- 1. System Administrator (no employee link - pure admin account)
INSERT INTO users (email, password_hash, role_id, employee_id, created_at) VALUES
    (
        'admin@company.com',
        '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS',
        1, -- ROLE_ADMIN
        NULL,
        NOW()
    )
ON CONFLICT (email) DO NOTHING;

-- 2. HR Manager - Leslie Andrews (can manage all employee)
INSERT INTO users (email, password_hash, role_id, employee_id, created_at) VALUES
    (
        'leslie@luv2code.com',
        '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS',
        3, -- ROLE_HR
        1,
        NOW()
    )
ON CONFLICT (email) DO NOTHING;

-- 3. IT Administrator - Emma Baumgarten (can manage user accounts)
INSERT INTO users (email, password_hash, role_id, employee_id, created_at) VALUES
    (
        'emma@luv2code.com',
        '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS',
        4, -- ROLE_IT
        2,
        NOW()
    )
ON CONFLICT (email) DO NOTHING;

-- 4. Regular Employee - Avani Gupta (can view own profile only)
INSERT INTO users (email, password_hash, role_id, employee_id, created_at) VALUES
    (
        'avani@luv2code.com',
        '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS',
        5, -- ROLE_EMPLOYEE
        3,
        NOW()
    )
ON CONFLICT (email) DO NOTHING;

-- Reset sequence to ensure proper ID generation
SELECT setval('users_id_seq', (SELECT MAX(id) FROM users));

-- ============================================================================
-- STEP 6: Verification Queries
-- ============================================================================
-- Uncomment these to verify the setup after running the script

-- Check all roles
-- SELECT * FROM app_role ORDER BY id;

-- Check all employee
-- SELECT id, first_name, last_name, email FROM employee ORDER BY id;

-- Check all users with their roles and employee info
-- SELECT
--     u.id,
--     u.email,
--     r.name as role,
--     COALESCE(e.first_name || ' ' || e.last_name, 'N/A') as employee_name
-- FROM users u
-- JOIN app_role r ON u.role_id = r.id
-- LEFT JOIN employee e ON u.employee_id = e.id
-- ORDER BY u.id;

-- ============================================================================
-- Setup Complete!
-- ============================================================================
-- Test Accounts (all use password: "password123"):
--   admin@company.com      - ROLE_ADMIN (System Admin)
--   leslie@luv2code.com    - ROLE_HR (HR Manager)
--   emma@luv2code.com      - ROLE_IT (IT Administrator)
--   avani@luv2code.com     - ROLE_EMPLOYEE (Regular Employee)
-- ============================================================================
