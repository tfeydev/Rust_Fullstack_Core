-- Seed Script: Initial Users
-- Description: Creates initial admin and test users for each role
--
-- Default password for all test accounts: "password123"
-- IMPORTANT: Change these passwords in production!

-- Note: These bcrypt hashes are for password "password123" with cost 12
-- Generated using: bcrypt::hash("password123", 12)

-- 1. Admin User (not linked to employee - system admin)
INSERT INTO users (email, password_hash, role_id, employee_id, created_at)
VALUES (
    'admin@company.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS',
    1, -- ROLE_ADMIN
    NULL,
    NOW()
) ON CONFLICT (email) DO NOTHING;

-- 2. HR User - Leslie Andrews (employee_id = 1)
INSERT INTO users (email, password_hash, role_id, employee_id, created_at)
VALUES (
    'leslie@luv2code.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS',
    3, -- ROLE_HR
    1,
    NOW()
) ON CONFLICT (email) DO NOTHING;

-- 3. IT User - Emma Baumgarten (employee_id = 2)
INSERT INTO users (email, password_hash, role_id, employee_id, created_at)
VALUES (
    'emma@luv2code.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS',
    4, -- ROLE_IT
    2,
    NOW()
) ON CONFLICT (email) DO NOTHING;

-- 4. Employee User - Avani Gupta (employee_id = 3)
INSERT INTO users (email, password_hash, role_id, employee_id, created_at)
VALUES (
    'avani@luv2code.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5BI6YOO2/R7kS',
    5, -- ROLE_EMPLOYEE
    3,
    NOW()
) ON CONFLICT (email) DO NOTHING;

-- Verify the inserted users
-- SELECT u.id, u.email, r.name as role, e.first_name, e.last_name
-- FROM users u
-- JOIN app_role r ON u.role_id = r.id
-- LEFT JOIN employee e ON u.employee_id = e.id;
