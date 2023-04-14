SELECT
  u.user_id AS buyer_id,
  join_date,
  count(order_date) AS orders_in_2019
FROM users AS u
LEFT OUTER JOIN
  orders AS o ON
  u.user_id = o.buyer_id
  AND year(order_date) = 2019
GROUP BY u.user_id
