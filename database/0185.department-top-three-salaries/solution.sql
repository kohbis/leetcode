select
    dep.Name as Department,
    emp.Name as Employee,
    emp.Salary as Salary
from
    Employee emp,
    Department dep
where
    emp.DepartmentId = dep.Id
    and (
        select
            count(distinct Salary)
        from
            Employee
        where
            Employee.DepartmentId = dep.Id
            and Salary > emp.Salary
    ) < 3
