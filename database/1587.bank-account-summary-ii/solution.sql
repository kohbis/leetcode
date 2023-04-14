SELECT
  u.name,
  sum(t.amount) AS balance
FROM users AS u
LEFT OUTER JOIN transactions AS t ON u.account = t.account
GROUP BY u.account
HAVING balance > 10000
