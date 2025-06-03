USE lichess;

CREATE TABLE final_type AS
SELECT
    g.GameId,
    g.FCId,

    CASE
        WHEN (
            fp.EndWhiteRooks = 1 AND fp.EndWhiteQueens = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhitePawns <= 1 AND
            fp.EndBlackRooks = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackPawns <= 1
        )
        OR (
            fp.EndBlackRooks = 1 AND fp.EndBlackQueens = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackPawns <= 1 AND
            fp.EndWhiteRooks = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhitePawns <= 1
        ) THEN 'rook_endings'

        WHEN (
            fp.EndWhiteQueens = 1 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhitePawns <= 1 AND
            fp.EndBlackQueens = 0 AND fp.EndBlackRooks = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackPawns <= 1
        )
        OR (
            fp.EndBlackQueens = 1 AND fp.EndBlackRooks = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackPawns <= 1 AND
            fp.EndWhiteQueens = 0 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhitePawns <= 1
        ) THEN 'queen_endings'

        WHEN (
            fp.EndWhiteBishops = 2 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhitePawns <= 1 AND
            fp.EndBlackBishops = 0 AND fp.EndBlackRooks = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackPawns <= 1
        )
        OR (
            fp.EndBlackBishops = 2 AND fp.EndBlackRooks = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackPawns <= 1 AND
            fp.EndWhiteBishops = 0 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhitePawns <= 1
        ) THEN 'two_bishops_endings'

        WHEN (
            fp.EndWhiteBishops = 1 AND fp.EndWhiteKnights = 1 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhitePawns <= 1 AND
            fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackRooks = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackPawns <= 1
        )
        OR (
            fp.EndBlackBishops = 1 AND fp.EndBlackKnights = 1 AND fp.EndBlackRooks = 0 AND fp.EndBlackQueens = 0 AND fp.EndBlackPawns <= 1 AND
            fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteQueens = 0 AND fp.EndWhitePawns <= 1
        ) THEN 'bishop_knight_endings'

        WHEN (
            fp.EndWhitePawns = 0 AND fp.EndWhiteKnights = 0 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteRooks = 0 AND fp.EndWhiteQueens = 0 AND
            fp.EndBlackPawns = 0 AND fp.EndBlackKnights = 0 AND fp.EndBlackBishops = 0 AND fp.EndBlackRooks = 0 AND fp.EndBlackQueens = 0
        ) THEN 'empty_endings'

        ELSE 'other'
    END AS endgame_type,

    CASE
        WHEN (
            fp.EndWhiteQueens = 1 AND fp.EndBlackQueens = 0
        ) OR (
            fp.EndWhiteRooks = 1 AND fp.EndBlackRooks = 0
        ) OR (
            fp.EndWhiteBishops = 2 AND fp.EndBlackBishops = 0
        ) OR (
            fp.EndWhiteBishops = 1 AND fp.EndWhiteKnights = 1 AND fp.EndBlackBishops = 0 AND fp.EndBlackKnights = 0
        ) THEN 'white'

        WHEN (
            fp.EndBlackQueens = 1 AND fp.EndWhiteQueens = 0
        ) OR (
            fp.EndBlackRooks = 1 AND fp.EndWhiteRooks = 0
        ) OR (
            fp.EndBlackBishops = 2 AND fp.EndWhiteBishops = 0
        ) OR (
            fp.EndBlackBishops = 1 AND fp.EndBlackKnights = 1 AND fp.EndWhiteBishops = 0 AND fp.EndWhiteKnights = 0
        ) THEN 'black'

        ELSE NULL
    END AS advantaged_player

FROM game g
JOIN piecesleft fp ON g.FCId = fp.FCId;


