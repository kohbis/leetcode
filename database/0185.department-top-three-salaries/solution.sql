SELECT dep.Name AS Department,
  emp.Name AS Employee,
  emp.Salary AS Salary
FROM Employee emp,
  Department dep
WHERE emp.DepartmentId = dep.Id
  AND (
    SELECT COUNT(DISTINCT Salary)
    FROM Employee
    WHERE Employee.DepartmentId = dep.Id
      AND Salary > emp.Salary
  ) < 3
