SELECT 
    DATE_FORMAT(g.DateTime, '%Y-%m') AS month,
    SUBSTRING_INDEX(o.name, ':', 1) AS opening_family,
    COUNT(*) AS frequency
FROM 
    game g
JOIN 
    opening o ON g.OpeningId = o.OpeningId
WHERE 
    SUBSTRING_INDEX(o.name, ':', 1) IN (
        'Sicilian Defense',
        'Queen''s Pawn Game',
        'French Defense',
        'Scandinavian Defense',
        'Italian Game',
        'Caro-Kann Defense',
        'King''s Pawn Game',
        'Queen''s Gambit Declined',
        'English Opening',
        'Modern Defense',
        'Zukertort Opening',
        'Philidor Defense',
        'Indian Game',
        'Russian Game',
        'Ruy Lopez',
        'Van''t Kruijs Opening',
        'Bishop''s Opening',
        'Hungarian Opening',
        'Pirc Defense',
        'Scotch Game'
    )
GROUP BY 
    month, opening_family
ORDER BY 
    month,
    FIELD(
        opening_family,
        'Sicilian Defense',
        'Queen''s Pawn Game',
        'French Defense',
        'Scandinavian Defense',
        'Italian Game',
        'Caro-Kann Defense',
        'King''s Pawn Game',
        'Queen''s Gambit Declined',
        'English Opening',
        'Modern Defense',
        'Zukertort Opening',
        'Philidor Defense',
        'Indian Game',
        'Russian Game',
        'Ruy Lopez',
        'Van''t Kruijs Opening',
        'Bishop''s Opening',
        'Hungarian Opening',
        'Pirc Defense',
        'Scotch Game'
    );
