USE lichess;

SELECT
    o.name AS opening_name,
    
  SUM(CASE WHEN ft.endgame_type = 'rook_endings' THEN 1 ELSE 0 END) AS rook_endings,
  SUM(CASE WHEN ft.endgame_type = 'queen_endings' THEN 1 ELSE 0 END) AS queen_endings,
  SUM(CASE WHEN ft.endgame_type = 'two_bishops_endings' THEN 1 ELSE 0 END) AS two_bishops_endings,
  SUM(CASE WHEN ft.endgame_type = 'bishop_knight_endings' THEN 1 ELSE 0 END) AS bishop_knight_endings,
  SUM(CASE WHEN ft.endgame_type = 'empty_endings' THEN 1 ELSE 0 END) AS empty_endings,

    COUNT(*) AS total_games

FROM 
    game g
JOIN 
    opening o ON g.OpeningId = o.OpeningId
JOIN
   final_type ft ON g.GameId = ft.GameId
WHERE
    o.name IN (
        'Sicilian Defense',
        'Queen''s Pawn Game',
        'French Defense',
        'Scandinavian Defense',
        'Italian Game',
        'Caro-Kann Defense',
        'King''s Pawn Game',
        'Queen''s Gambit Declined',
        'English Opening',
        'Modern Defense'
    )
GROUP BY 
    o.name
ORDER BY
    FIELD(o.name,
        'Sicilian Defense',
        'Queen''s Pawn Game',
        'French Defense',
        'Scandinavian Defense',
        'Italian Game',
        'Caro-Kann Defense',
        'King''s Pawn Game',
        'Queen''s Gambit Declined',
        'English Opening',
        'Modern Defense'
    );