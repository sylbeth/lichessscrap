USE lichess;

SELECT
  CASE
    WHEN ABS(CAST(WhiteElo AS SIGNED) - CAST(BlackElo AS SIGNED)) < 50 THEN '< 50'
    WHEN ABS(CAST(WhiteElo AS SIGNED) - CAST(BlackElo AS SIGNED)) BETWEEN 50 AND 99 THEN '50–99'
    WHEN ABS(CAST(WhiteElo AS SIGNED) - CAST(BlackElo AS SIGNED)) BETWEEN 100 AND 199 THEN '100–199'
    WHEN ABS(CAST(WhiteElo AS SIGNED) - CAST(BlackElo AS SIGNED)) BETWEEN 200 AND 299 THEN '200–299'
    WHEN ABS(CAST(WhiteElo AS SIGNED) - CAST(BlackElo AS SIGNED)) BETWEEN 300 AND 399 THEN '300–399'
    ELSE '400+'
  END AS elo_diff_range,

  COUNT(*) AS total_games,

  SUM(CASE
      WHEN WhiteElo > BlackElo AND Result = 'White' THEN 1
      WHEN BlackElo > WhiteElo AND Result = 'Black' THEN 1
      ELSE 0
  END) AS higher_elo_wins,

  ROUND(
    100.0 * SUM(CASE
      WHEN WhiteElo > BlackElo AND Result = 'White' THEN 1
      WHEN BlackElo > WhiteElo AND Result = 'Black' THEN 1
      ELSE 0
    END) / COUNT(*), 2
  ) AS win_pct_higher_elo

FROM game
WHERE Result IN ('White', 'Black')
  AND WhiteElo IS NOT NULL AND BlackElo IS NOT NULL
GROUP BY elo_diff_range
ORDER BY 
  CASE elo_diff_range
    WHEN '< 50' THEN 1
    WHEN '50–99' THEN 2
    WHEN '100–199' THEN 3
    WHEN '200–299' THEN 4
    WHEN '300–399' THEN 5
    WHEN '400+' THEN 6
  END;

