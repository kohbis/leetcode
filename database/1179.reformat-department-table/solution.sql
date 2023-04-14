SELECT
  id,
  sum(
    CASE
      WHEN month = 'Jan' THEN revenue
    END
  ) AS jan_revenue,
  sum(
    CASE
      WHEN month = 'Feb' THEN revenue
    END
  ) AS feb_revenue,
  sum(
    CASE
      WHEN month = 'Mar' THEN revenue
    END
  ) AS mar_revenue,
  sum(
    CASE
      WHEN month = 'Apr' THEN revenue
    END
  ) AS apr_revenue,
  sum(
    CASE
      WHEN month = 'May' THEN revenue
    END
  ) AS may_revenue,
  sum(
    CASE
      WHEN month = 'Jun' THEN revenue
    END
  ) AS jun_revenue,
  sum(
    CASE
      WHEN month = 'Jul' THEN revenue
    END
  ) AS jul_revenue,
  sum(
    CASE
      WHEN month = 'Aug' THEN revenue
    END
  ) AS aug_revenue,
  sum(
    CASE
      WHEN month = 'Sep' THEN revenue
    END
  ) AS sep_revenue,
  sum(
    CASE
      WHEN month = 'Oct' THEN revenue
    END
  ) AS oct_revenue,
  sum(
    CASE
      WHEN month = 'Nov' THEN revenue
    END
  ) AS nov_revenue,
  sum(
    CASE
      WHEN month = 'Dec' THEN revenue
    END
  ) AS dec_revenue
FROM department
GROUP BY id
ORDER BY id;
