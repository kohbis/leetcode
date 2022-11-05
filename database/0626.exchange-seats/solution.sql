SELECT (CASE
            WHEN MOD(id, 2) != 0
                 AND seat_count.counts = id THEN id
            WHEN MOD(id, 2) != 0
                 AND seat_count.counts != id THEN id + 1
            ELSE id - 1
        END) AS id,
       student
FROM seat,

  (SELECT COUNT(*) AS counts
   FROM seat) seat_count
ORDER BY id ASC
