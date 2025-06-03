USE lichess;

SELECT
    rs.Name AS ruleset_name,

    -- Finals concrets
    SUM(CASE WHEN ft.endgame_type = 'rook_endings' THEN 1 ELSE 0 END) AS rook_endings,
	SUM(CASE WHEN ft.endgame_type = 'queen_endings' THEN 1 ELSE 0 END) AS queen_endings,
	SUM(CASE WHEN ft.endgame_type = 'two_bishops_endings' THEN 1 ELSE 0 END) AS two_bishops_endings,
	SUM(CASE WHEN ft.endgame_type = 'bishop_knight_endings' THEN 1 ELSE 0 END) AS bishop_knight_endings,
	SUM(CASE WHEN ft.endgame_type = 'empty_endings' THEN 1 ELSE 0 END) AS empty_endings,

    COUNT(*) AS total_games

FROM 
    game g
JOIN 
    ruleset rs ON g.RuleSetId = rs.RuleSetId
JOIN 
    final_type ft ON ft.GameId = g.GameId
WHERE
    g.WhiteElo IS NOT NULL
    AND g.BlackElo IS NOT NULL
    AND rs.Name IN (
        'Rated Blitz',
        'Rated Bullet',
        'Rated Rapid',
        'Rated UltraBullet',
        'Rated Classical',
        'Rated Correspondence'
    )
GROUP BY 
    rs.Name
ORDER BY
    FIELD(rs.Name,
        'Rated Blitz',
        'Rated Bullet',
        'Rated Rapid',
        'Rated UltraBullet',
        'Rated Classical',
        'Rated Correspondence'
    );