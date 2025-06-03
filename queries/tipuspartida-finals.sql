USE lichess;

SELECT
    rs.Name AS ruleset_name,

    -- Finals concrets
    SUM((
        (fp.EndWhiteRooks = 1 AND fp.EndWhiteQueens = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhitePawns <= 1 AND
         fp.EndBlackRooks = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackPawns <= 1)
        OR
        (fp.EndBlackRooks = 1 AND fp.EndBlackQueens = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackPawns <= 1 AND
         fp.EndWhiteRooks = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhitePawns <= 1)
    )) AS rook_endings,

    SUM((
        (fp.EndWhiteQueens = 1 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhitePawns <= 1 AND
         fp.EndBlackQueens = 0 AND fp.EndBlackRooks = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackPawns <= 1)
        OR
        (fp.EndBlackQueens = 1 AND fp.EndBlackRooks = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackPawns <= 1 AND
         fp.EndWhiteQueens = 0 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhitePawns <= 1)
    )) AS queen_endings,

    SUM((
        (fp.EndWhiteBishops = 2 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhitePawns <= 1 AND
         fp.EndBlackBishops = 0 AND fp.EndBlackRooks = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackPawns <= 1)
        OR
        (fp.EndBlackBishops = 2 AND fp.EndBlackRooks = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackPawns <= 1 AND
         fp.EndWhiteBishops = 0 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhitePawns <= 1)
    )) AS two_bishops_endings,

    SUM((
        (fp.EndWhiteBishops = 1 AND fp.EndWhiteKnights = 1 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhitePawns <= 1 AND
         fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackRooks = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackPawns <= 1)
        OR
        (fp.EndBlackBishops = 1 AND fp.EndBlackKnights = 1 AND fp.EndBlackRooks = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackPawns <= 1 AND
         fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhitePawns <= 1)
    )) AS bishop_knight_endings,

    SUM((
        fp.EndWhitePawns = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteQueens = 0 AND
        fp.EndBlackPawns = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackRooks = 0 AND fp.EndBlackQueens = 0
    )) AS empty_endings,

    COUNT(*) AS total_games

FROM 
    game g
JOIN 
    ruleset rs ON g.RuleSetId = rs.RuleSetId
JOIN 
    piecesleft fp ON g.FCId = fp.FCId
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