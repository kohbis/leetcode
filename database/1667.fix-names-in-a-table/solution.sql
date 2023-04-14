SELECT
  user_id,
  concat(
    substring(upper(name), 1, 1),
    substring(lower(name), 2)
  ) AS name
FROM users
ORDER BY user_id
