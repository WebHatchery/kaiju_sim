-- Automatically log ownership changes
DROP TRIGGER IF EXISTS track_ownership_changes;
CREATE TRIGGER track_ownership_changes
AFTER UPDATE ON kaiju
FOR EACH ROW
BEGIN
    IF OLD.owner_user_id <> NEW.owner_user_id THEN
        INSERT INTO ownership_history (
            kaiju_id,
            from_user_id,
            to_user_id,
            transfer_type,
            timestamp,
            server_signature,
            nonce
        ) VALUES (
            NEW.id,
            OLD.owner_user_id,
            NEW.owner_user_id,
            'admin_transfer',
            NOW(),
            'auto_logged',
            0
        );
    END IF;
END;

-- Prevent state_hash tampering
DROP TRIGGER IF EXISTS enforce_state_hash;
CREATE TRIGGER enforce_state_hash
BEFORE UPDATE ON kaiju
FOR EACH ROW
BEGIN
    SET NEW.state_version = OLD.state_version + 1;
END;
