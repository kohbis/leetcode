SELECT event_day AS DAY,
       emp_id,
       sum(out_time - in_time) AS total_time
FROM Employees
GROUP BY emp_id,
         event_day
