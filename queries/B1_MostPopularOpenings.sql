SELECT 
    SUBSTRING_INDEX(o.name, ':', 1) AS opening_family,
    COUNT(*) AS frequency
FROM 
    game g
JOIN 
    opening o ON g.OpeningId = o.OpeningId
GROUP BY 
	opening_family
ORDER BY 
    frequency DESC
LIMIT 20;