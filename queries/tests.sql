--! get_user_candidate_by_username
SELECT * FROM user_candidates
WHERE username = :username;
