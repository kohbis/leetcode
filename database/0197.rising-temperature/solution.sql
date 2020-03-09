select
    t.Id
from
    Weather t,
    Weather y
where
    datediff(t.RecordDate, y.RecordDate) = 1
    and t.Temperature > y.Temperature;
