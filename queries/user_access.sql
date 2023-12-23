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

--! get_user_system_notifications : (system_notifications_id?)
SELECT s.id, s.text, s.users_id, s.created_at, views.system_notifications_id
FROM system_notifications s
LEFT JOIN views
ON views.system_notifications_id = s.id
RIGHT JOIN users
ON users.id = s.users_id
ORDER BY s.created_at DESC;

--! set_system_notification_have_been_seen
INSERT INTO views (users_id, system_notifications_id)
VALUES (:user_id, :system_notification_id);
