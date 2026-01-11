-- Insert initial server signature key placeholder
INSERT IGNORE INTO server_signature_keys (key_name, public_key, algorithm, purpose)
VALUES
    ('ownership-key-v1', 'PLACEHOLDER_WILL_BE_REPLACED', 'secp256k1', 'ownership');

-- Create system user for genesis kaiju
INSERT IGNORE INTO users (id, username, email, is_active)
VALUES ('00000000-0000-0000-0000-000000000000', 'system', 'system@kaiju.game', TRUE);
