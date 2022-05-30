SELECT stock_name,
       sum(CASE
               WHEN OPERATION = 'Sell' THEN price
           END) - sum(CASE
                          WHEN OPERATION = 'Buy' THEN price
                      END) AS capital_gain_loss
FROM stocks
GROUP BY stock_name
