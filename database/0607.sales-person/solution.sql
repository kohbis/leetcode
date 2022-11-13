SELECT s.name
FROM SalesPerson s
WHERE s.sales_id not in (
        SELECT o.sales_id
        FROM orders o
            INNER JOIN company c ON o.com_id = c.com_id
        WHERE c.name = 'RED'
    )
