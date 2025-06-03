USE lichess;

SELECT
    opening_family,
    SUM(CASE WHEN elo_category = 'Under 1400' THEN frequency ELSE 0 END) AS `Under 1400`,
    SUM(CASE WHEN elo_category = '1400–1599' THEN frequency ELSE 0 END) AS `1400-1599`,
    SUM(CASE WHEN elo_category = '1600–1799' THEN frequency ELSE 0 END) AS `1600-1799`,
    SUM(CASE WHEN elo_category = '1800–2199' THEN frequency ELSE 0 END) AS `1800-2199`,
    SUM(CASE WHEN elo_category = '2200 and above' THEN frequency ELSE 0 END) AS `2200+`
FROM (
    SELECT
        SUBSTRING_INDEX(o.name, ':', 1) AS opening_family,
        CASE
            WHEN (g.WhiteElo + g.BlackElo)/2 < 1400 THEN 'Under 1400'
            WHEN (g.WhiteElo + g.BlackElo)/2 BETWEEN 1400 AND 1599 THEN '1400–1599'
            WHEN (g.WhiteElo + g.BlackElo)/2 BETWEEN 1600 AND 1799 THEN '1600–1799'
            WHEN (g.WhiteElo + g.BlackElo)/2 BETWEEN 1800 AND 2199 THEN '1800–2199'
            ELSE '2200 and above'
        END AS elo_category,
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
        opening_family, elo_category
) AS sub
GROUP BY opening_family
ORDER BY FIELD(opening_family,
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
