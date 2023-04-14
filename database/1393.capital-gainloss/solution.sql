SELECT
  stock_name,
  sum(
    CASE
      WHEN operation = 'Sell' THEN price
    END
  ) - sum(
    CASE
      WHEN operation = 'Buy' THEN price
    END
  ) AS capital_gain_loss
FROM stocks
GROUP BY stock_name
