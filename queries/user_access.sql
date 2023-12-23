--! get_user_settings
SELECT inbox_messages, order_messages, order_updates
FROM user_settings
JOIN users
ON users.user_settings_id = user_settings.id
WHERE users.id = :user_id;

--! set_user_settings
UPDATE user_settings
SET inbox_messages = :inbox_messages, order_messages = :order_messages, order_updates = :order_updates
WHERE id = (
    SELECT user_settings_id
    FROM users
    WHERE id = :id
);
