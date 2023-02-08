-- name: median_salary?
-- # Parameters
-- param: from_date : &sibyl::Date - First date of the hire range
-- param: thru_date : &sibyl::Date - Last date of the hire range
-- param: country_ids : &str - List of country codes
--
SELECT c.country_name, Median(e.salary)
  FROM hr.employees e
  JOIN hr.departments d ON d.department_id = e.department_id
  JOIN hr.locations l   ON l.location_id = d.location_id
  JOIN hr.countries c   ON c.country_id = l.country_id
 WHERE e.hire_date BETWEEN :FROM_DATE AND :THRU_DATE
   AND c.country_id IN (:COUNTRY_IDS)
 GROUP BY c.country_name;
