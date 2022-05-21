SELECT u.name,
       sum(t.amount) AS balance
FROM Users u
LEFT OUTER JOIN Transactions t ON u.account = t.account
GROUP BY u.account
HAVING balance > 10000
