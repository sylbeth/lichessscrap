DROP VIEW IF EXISTS PiecesLeft;
CREATE VIEW PiecesLeft AS
SELECT FCId, (EndWhitePieces >> 12) & 0xF as EndWhitePawns, (EndWhitePieces >> 9) & 0x7 as EndWhiteKnights, (EndWhitePieces >> 6) & 0x7 as EndWhiteBishops, (EndWhitePieces >> 3) & 0x7 as EndWhiteRooks, EndWhitePieces & 0x7 as EndWhiteQueens, (EndBlackPieces >> 12) & 0xF as EndBlackPawns, (EndBlackPieces >> 9) & 0x7 as EndBlackKnights, (EndBlackPieces >> 6) & 0x7 as EndBlackBishops, (EndBlackPieces >> 3) & 0x7 as EndBlackRooks, EndBlackPieces & 0x7 as EndBlackQueens
FROM FinalConfiguration;
