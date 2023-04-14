SELECT
  s.product_id,
  p.product_name
FROM sales AS s
INNER JOIN product AS p ON s.product_id = p.product_id
GROUP BY s.product_id
HAVING
  MIN(s.sale_date) >= '2019-01-01'
  AND MAX(s.sale_date) <= '2019-03-31'
