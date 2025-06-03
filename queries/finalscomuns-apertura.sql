USE lichess;

SELECT
    o.name AS opening_name,
    
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
            fp.EndWhitePawns <= 1 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteQueens = 0 AND
            fp.EndBlackPawns <= 1 AND fp.EndBlackKnights = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackRooks = 0 AND fp.EndBlackQueens = 0
        )
    ) AS empty_endings,

    COUNT(*) AS total_games

FROM 
    game g
JOIN 
    opening o ON g.OpeningId = o.OpeningId
JOIN
    piecesleft fp ON g.FCId = fp.FCId
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