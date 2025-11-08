-- Migration: Add employee_id to users table
-- Description: Links user accounts to employee records

-- UP Migration
ALTER TABLE users
ADD COLUMN employee_id INTEGER;

-- Add foreign key constraint
ALTER TABLE users
ADD CONSTRAINT fk_users_employee
FOREIGN KEY (employee_id)
REFERENCES employee(id)
ON DELETE SET NULL;

-- Add index for better performance
CREATE INDEX idx_users_employee_id ON users(employee_id);

-- Optional: Add unique constraint if one employee should only have one user account
-- ALTER TABLE users ADD CONSTRAINT unique_employee_id UNIQUE (employee_id);

-- DOWN Migration (if you need to rollback)
-- ALTER TABLE users DROP CONSTRAINT fk_users_employee;
-- DROP INDEX idx_users_employee_id;
-- ALTER TABLE users DROP COLUMN employee_id;
