SELECT patient_id,
       patient_name,
       conditions
FROM patients
WHERE conditions like "DIAB1%"
  OR conditions like "% DIAB1%"
