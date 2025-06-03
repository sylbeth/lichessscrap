USE lichess;

SELECT
    CASE
        WHEN (g.WhiteElo + g.BlackElo)/2 < 1400 THEN 'Under 1400'
        WHEN (g.WhiteElo + g.BlackElo)/2 BETWEEN 1400 AND 1599 THEN '1400–1599'
        WHEN (g.WhiteElo + g.BlackElo)/2 BETWEEN 1600 AND 1799 THEN '1600–1799'
        WHEN (g.WhiteElo + g.BlackElo)/2 BETWEEN 1800 AND 2199 THEN '1800–2199'
        ELSE '2200 and above'
    END AS elo_category,

    -- Final de torre
    SUM(
        (
            (fp.EndWhiteRooks = 1 AND fp.EndWhiteQueens = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhitePawns <= 1 AND
             fp.EndBlackRooks = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackPawns <= 1)
            OR
            (fp.EndBlackRooks = 1 AND fp.EndBlackQueens = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackPawns <= 1 AND
             fp.EndWhiteRooks = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhitePawns <= 1)
        )
    ) AS rook_endings,

    -- Final de dama
    SUM(
        (
            (fp.EndWhiteQueens = 1 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhitePawns <= 1 AND
             fp.EndBlackQueens = 0 AND fp.EndBlackRooks = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackPawns <= 1)
            OR
            (fp.EndBlackQueens = 1 AND fp.EndBlackRooks = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackPawns <= 1 AND
             fp.EndWhiteQueens = 0 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhitePawns <= 1)
        )
    ) AS queen_endings,

    -- Final de dos alfiles
    SUM(
        (
            (fp.EndWhiteBishops = 2 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhitePawns <= 1 AND
             fp.EndBlackRooks = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackPawns <= 1 AND fp.EndBlackBishops = 0)
            OR
            (fp.EndBlackBishops = 2 AND fp.EndBlackRooks = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackPawns <= 1 AND
             fp.EndWhiteRooks = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhitePawns <= 1 AND fp.EndWhiteBishops = 0)
        )
    ) AS two_bishops_endings,

    -- Final de alfil + caballo
    SUM(
        (
            (fp.EndWhiteBishops = 1 AND fp.EndWhiteKnights = 1 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhitePawns <= 1 AND
             fp.EndBlackRooks = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackPawns <= 1 AND fp.EndBlackBishops = 0)
            OR
            (fp.EndBlackBishops = 1 AND fp.EndBlackKnights = 1 AND fp.EndBlackRooks = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackPawns <= 1 AND
             fp.EndWhiteRooks = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhitePawns <= 1 AND fp.EndWhiteBishops = 0)
        )
    ) AS bishop_knight_endings,

    -- Final vacío
    SUM(
        (
            fp.EndWhitePawns = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteQueens = 0 AND
            fp.EndBlackPawns = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackRooks = 0 AND fp.EndBlackQueens = 0
        )
    ) AS empty_endings,

    COUNT(*) AS total_games

FROM 
    game g
JOIN 
    piecesleft fp ON g.FCId = fp.FCId
WHERE
    g.WhiteElo IS NOT NULL AND g.BlackElo IS NOT NULL
GROUP BY 
    elo_category
ORDER BY
    FIELD(elo_category, 'Under 1400', '1400–1599', '1600–1799', '1800–2199', '2200 and above');