SELECT t.id
FROM weather AS t,
  weather AS y
WHERE
  datediff(t.recorddate, y.recorddate) = 1
  AND t.temperature > y.temperature;
