SELECT name
FROM customer
WHERE customer.referee_id IS NULL
  OR customer.referee_id != 2
