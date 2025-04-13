USE `lichess`;

INSERT INTO RuleSet (Name, URLId, Kind, TimeControl) 
VALUES 
  ({Name}, {URLId}, {Kind}, {TimeControl});

INSERT INTO Player (Name)
VALUES 
  ({PlayerName});

INSERT INTO Opening (Name, EcoLetter, EcoNumber) 
VALUES 
  ({OpeningName}, {EcoLetter}, {EcoNumber});

INSERT INTO FinalConfiguration (FEN, EndPieces) 
VALUES 
  ({FEN}, {EndPieces});

INSERT INTO Game (GameId, RuleSetId, OpeningId, FCId, White, WhiteElo, WhiteTitle, Black, BlackElo, BlackTitle, StartTime, Increment, Result, Termination, DateTime, HasClock, HasEvaluations) 
VALUES 
  ({GameId}, {RuleSetId}, {OpeningId}, {FCId}, {White}, {WhiteElo}, {WhiteTitle}, {Black}, {BlackElo}, {BlackTitle}, {StartTime}, {Increment}, {Result}, {Termination}, '{DateTime}', {HasClock}, {HasEvaluations});
