SELECT
    CASE
        WHEN (g.WhiteElo + g.BlackElo)/2 < 1400 THEN 'Under 1400'
        WHEN (g.WhiteElo + g.BlackElo)/2 BETWEEN 1400 AND 1599 THEN '1400-1599'
        WHEN (g.WhiteElo + g.BlackElo)/2 BETWEEN 1600 AND 1799 THEN '1600-1799'
        WHEN (g.WhiteElo + g.BlackElo)/2 BETWEEN 1800 AND 2199 THEN '1800-2199'
        ELSE '2200 and above'
    END AS elo_category,

    SUM(CASE WHEN ft.endgame_type = 'rook_endings' THEN 1 ELSE 0 END) AS rook_endings,
    SUM(CASE WHEN ft.endgame_type = 'queen_endings' THEN 1 ELSE 0 END) AS queen_endings,
    SUM(CASE WHEN ft.endgame_type = 'two_bishops_endings' THEN 1 ELSE 0 END) AS two_bishops_endings,
    SUM(CASE WHEN ft.endgame_type = 'bishop_knight_endings' THEN 1 ELSE 0 END) AS bishop_knight_endings,
    SUM(CASE WHEN ft.endgame_type = 'empty_endings' THEN 1 ELSE 0 END) AS empty_endings,

    COUNT(*) AS total_games
FROM 
    game g
JOIN 
    final_type ft ON ft.GameId = g.GameId
WHERE
    g.WhiteElo IS NOT NULL AND g.BlackElo IS NOT NULL
GROUP BY 
    elo_category
ORDER BY
    FIELD(elo_category, 'Under 1400', '1400-1599', '1600-1799', '1800-2199', '2200 and above');