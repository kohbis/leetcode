SELECT
  P.FirstName,
  P.LastName,
  P.City,
  P.State
FROM Person AS P
LEFT JOIN Address AS A ON P.PersonId = A.PersonId;
