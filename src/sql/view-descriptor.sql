DROP VIEW IF EXISTS MoveDescriptor;
CREATE VIEW MoveDescriptor AS
SELECT GameId, Num, Descriptor >> 31 as IsWhite, (Descriptor >> 28) & 0x7 as MovedPiece, (Descriptor >> 25) & 0x7 as FromRow, ((Descriptor >> 22) & 0x7) as FromColumn, (Descriptor >> 19) & 0x7 as CapturedPiece, (Descriptor >> 16) & 0x7 as ToRow, ((Descriptor >> 13) & 0x7) as ToColumn, (Descriptor >> 10) & 0x7 as PromotedPiece, (Descriptor >> 9) & 0x1 as IsCheck, (Descriptor >> 8) & 0x1 as IsMate, (Descriptor & 0xFF) as NAG
FROM Move;
