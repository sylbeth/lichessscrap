SELECT 
    DATE_FORMAT(g.DateTime, '%Y-%m') AS month,
    SUBSTRING_INDEX(o.name, ':', 1) AS opening_family,
    COUNT(*) AS frequency
FROM 
    game g
JOIN 
    opening o ON g.OpeningId = o.OpeningId
GROUP BY 
    month, opening_family
ORDER BY 
    frequency DESC;