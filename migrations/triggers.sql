-- ================================================
--  CLEANUP: remove old triggers & functions
-- ================================================

DROP TRIGGER IF EXISTS trg_sync_employee_email ON public.employee;
DROP TRIGGER IF EXISTS trg_sync_user_email ON public.users;
DROP TRIGGER IF EXISTS trg_prevent_duplicate_user_emails ON public.users;

DROP FUNCTION IF EXISTS public.sync_employee_email_to_users() CASCADE;
DROP FUNCTION IF EXISTS public.sync_user_email_to_employee() CASCADE;
DROP FUNCTION IF EXISTS public.prevent_duplicate_user_emails() CASCADE;

-- ================================================
--  FUNCTION: Employee → Users email sync (guarded)
-- ================================================
CREATE OR REPLACE FUNCTION public.sync_employee_email_to_users()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    -- Prevent recursion using transaction guard
    IF coalesce(current_setting('app.sync_guard', true), '0') = '1' THEN
        RETURN NEW;
    END IF;

    -- Enable guard for this transaction
    PERFORM set_config('app.sync_guard', '1', true);

    -- Update linked user records
    UPDATE public.users
    SET email = NEW.email
    WHERE employee_id = NEW.id
      AND (email IS DISTINCT FROM NEW.email);

    RETURN NEW;
END;
$$;

-- ================================================
--  TRIGGER: Employee email changes
-- ================================================
CREATE TRIGGER trg_sync_employee_email
AFTER UPDATE OF email ON public.employee
FOR EACH ROW
WHEN (NEW.email IS DISTINCT FROM OLD.email)
EXECUTE FUNCTION public.sync_employee_email_to_users();

-- ================================================
--  FUNCTION: Users → Employee email sync (guarded)
-- ================================================
CREATE OR REPLACE FUNCTION public.sync_user_email_to_employee()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    -- Prevent recursion using transaction guard
    IF coalesce(current_setting('app.sync_guard', true), '0') = '1' THEN
        RETURN NEW;
    END IF;

    -- Only sync if user is linked to an employee
    IF NEW.employee_id IS NULL THEN
        RETURN NEW;
    END IF;

    -- Enable guard for this transaction
    PERFORM set_config('app.sync_guard', '1', true);

    -- Update employee record
    UPDATE public.employee
    SET email = NEW.email
    WHERE id = NEW.employee_id
      AND (email IS DISTINCT FROM NEW.email);

    RETURN NEW;
END;
$$;

-- ================================================
--  TRIGGER: Users email changes
-- ================================================
CREATE TRIGGER trg_sync_user_email
AFTER UPDATE OF email ON public.users
FOR EACH ROW
WHEN (NEW.email IS DISTINCT FROM OLD.email)
EXECUTE FUNCTION public.sync_user_email_to_employee();

-- ================================================
--  FUNCTION: Prevent duplicate emails in users
-- ================================================
CREATE OR REPLACE FUNCTION public.prevent_duplicate_user_emails()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    -- Check if another user already has this email
    IF EXISTS (
        SELECT 1
        FROM public.users
        WHERE email = NEW.email
          AND id <> COALESCE(NEW.id, -1)
    ) THEN
        RAISE EXCEPTION 'EMAIL_EXISTS';
    END IF;

    RETURN NEW;
END;
$$;

-- ================================================
--  TRIGGER: Prevent duplicate email on INSERT/UPDATE
-- ================================================
CREATE TRIGGER trg_prevent_duplicate_user_emails
BEFORE INSERT OR UPDATE OF email ON public.users
FOR EACH ROW
EXECUTE FUNCTION public.prevent_duplicate_user_emails();
