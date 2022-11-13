SELECT employee_id
FROM employees
WHERE employee_id not in (
        SELECT employee_id
        FROM salaries
    )
UNION
SELECT employee_id
FROM salaries
WHERE employee_id not in (
        SELECT employee_id
        FROM employees
    )
ORDER BY employee_id
