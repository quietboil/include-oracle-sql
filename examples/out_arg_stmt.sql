-- name: new_deparment!
-- # Parameters
-- param: department_name : &str - Name of the new department
-- param: city : &str - City where the department is located
-- param: department_id : &mut u32 - ID of the new department
--
INSERT INTO hr.departments
     ( department_id, department_name, location_id )
VALUES
     ( hr.departments_seq.NextVal, :DEPARTMENT_NAME
     , (SELECT location_id FROM hr.locations WHERE city = :CITY)
     )
RETURN department_id INTO :DEPARTMENT_ID
