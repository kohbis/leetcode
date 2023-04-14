SELECT
  t.request_at AS day,
  round(sum(t.status != "completed") / count(*), 2) AS "Cancellation Rate"
FROM trips AS t
INNER JOIN
  users AS u ON
  t.client_id = u.users_id
  AND u.banned = "No"
WHERE t.request_at BETWEEN "2013-10-01" AND "2013-10-03"
GROUP BY t.request_at
