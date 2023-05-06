SELECT
  employees.unique_id,
  employeeuni.name
FROM
  employees
  LEFT OUTER JOIN employeeuni ON employees.id = employeeuni.id
