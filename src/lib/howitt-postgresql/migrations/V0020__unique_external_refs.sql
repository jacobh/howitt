CREATE UNIQUE INDEX rides_external_ref_unique 
ON rides ((external_ref->'id'))
WHERE external_ref->'id' IS NOT NULL;

CREATE UNIQUE INDEX routes_external_ref_unique 
ON routes ((external_ref->'id'))
WHERE external_ref->'id' IS NOT NULL;
