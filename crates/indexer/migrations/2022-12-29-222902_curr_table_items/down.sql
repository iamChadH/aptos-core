-- This file should undo anything in `up.sql`
ALTER TABLE events DROP COLUMN IF EXISTS event_index;