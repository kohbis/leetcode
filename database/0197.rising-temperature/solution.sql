SELECT t.Id
FROM Weather t,
     Weather y
WHERE datediff(t.RecordDate, y.RecordDate) = 1
  AND t.Temperature > y.Temperature;