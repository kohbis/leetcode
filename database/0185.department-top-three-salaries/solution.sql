SELECT
  dep.name AS department,
  emp.name AS employee,
  emp.salary AS salary
FROM employee AS emp,
  department AS dep
WHERE
  emp.departmentid = dep.id
  AND (
    SELECT COUNT(DISTINCT employee.salary)
    FROM employee
    WHERE
      employee.departmentid = dep.id
      AND employee.salary > emp.salary
  ) < 3
