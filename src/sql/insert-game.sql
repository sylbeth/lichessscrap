INSERT INTO Game (RuleSetId, OpeningId, FCId, White, WhiteElo, WhiteTitle, Black, BlackElo, BlackTitle, StartTime, Increment, Result, Termination, DateTime, HasClock, HasEvaluations)
VALUES (:ruleset_id, :opening_id, :fc_id, :white, :white_elo, :white_title, :black, :black_elo, :black_title, :start_time, :increment, :result, :termination, :datetime, :has_clock, :has_evaluations);
