SELECT
  sell_date,
  count(DISTINCT product) AS num_sold,
  group_concat(DISTINCT product) AS products
FROM activities
GROUP BY sell_date
