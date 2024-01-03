-- SELECTING CONTENT --

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

--! get_conversation_by_user_id : (conversations_id?)
SELECT c.id AS conversations_id
FROM conversations c
JOIN participants p1 ON c.id = p1.conversations_id AND p1.users_id = :first_user_id
JOIN participants p2 ON c.id = p2.conversations_id AND p2.users_id = :second_user_id;

--! get_conversations_entries
SELECT 
    interlocutor.username AS interlocutor,
    last_message.text AS last_message_text,
    last_message.created_at AS last_message_timestamp,
    interlocutor_avatar.key AS image_url,
    (SELECT COUNT(*) 
        FROM messages 
        WHERE messages.conversations_id = conversations.id 
        AND messages.id NOT IN (SELECT messages_id FROM views WHERE users_id = :user_id)
    ) AS unread_messages_count
FROM 
    conversations
JOIN 
    participants ON participants.conversations_id = conversations.id
JOIN 
    users AS interlocutor ON participants.users_id = interlocutor.id AND interlocutor.id != :user_id
LEFT JOIN 
    objects AS interlocutor_avatar ON interlocutor_avatar.avatar_users_id = interlocutor.id
LEFT JOIN LATERAL
    (SELECT m1.*
        FROM messages m1
        WHERE m1.conversations_id = conversations.id
        ORDER BY m1.created_at DESC
        LIMIT 1
    ) last_message ON TRUE
WHERE 
    conversations.id IN (SELECT conversations_id FROM participants WHERE users_id = :user_id);

-- UPDATING CONTENT --

--! create_new_conversation
INSERT INTO conversations VALUES (DEFAULT) returning id;

--! add_participants_to_conversation
INSERT INTO participants (conversations_id, users_id)
VALUES
    (:conversation_id, :user1),
    (:conversation_id, :user2);

--! insert_new_message (service_id?, reply_message_id?)
INSERT INTO messages (conversations_id, services_id, users_id, messages_id, text)
VALUES (:conversation_id, :service_id, :user_id, :reply_message_id, :text);
