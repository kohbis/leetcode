SELECT id,
       CASE
              WHEN p_id IS NULL THEN 'Root'
              WHEN id in (
                     SELECT p_id
                     FROM tree
              ) THEN 'Inner'
              ELSE 'Leaf'
       END AS TYPE
FROM tree
ORDER BY id ASC
