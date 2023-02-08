-- name: prepare_median_salary_query.
-- Prepares a statement that queries median salary of all countries
-- in the specified region.
SELECT c.country_name, Median(e.salary)
  FROM hr.employees e
  JOIN hr.departments d ON d.department_id = e.department_id
  JOIN hr.locations l   ON l.location_id = d.location_id
  JOIN hr.countries c   ON c.country_id = l.country_id
  JOIN hr.regions r     ON r.region_id = c.region_id
 WHERE r.region_name = :REGION_NAME
 GROUP BY c.country_name
 ORDER BY 2 DESC, 1;
