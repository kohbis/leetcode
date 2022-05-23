SELECT employee_id,
       if(employee_id % 2 = 0
          OR name like 'M%', 0, salary) AS bonus
FROM Employees
