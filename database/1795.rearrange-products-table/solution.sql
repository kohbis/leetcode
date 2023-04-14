SELECT
  t.product_id,
  t.store,
  t.price
FROM (
  SELECT
    product_id,
    'store1' AS store,
    store1 AS price
  FROM products
  UNION DISTINCT
  SELECT
    product_id,
    'store2' AS store,
    store2 AS price
  FROM products
  UNION DISTINCT
  SELECT
    product_id,
    'store3' AS store,
    store3 AS price
  FROM products
) AS t
WHERE t.price IS NOT NULL
