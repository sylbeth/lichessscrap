SELECT
  ft.endgame_type,
  ft.advantaged_player,
  
	CASE
        WHEN (g.WhiteElo + g.BlackElo)/2 < 1400 THEN 'Under 1400'
        WHEN (g.WhiteElo + g.BlackElo)/2 BETWEEN 1400 AND 1599 THEN '1400–1599'
        WHEN (g.WhiteElo + g.BlackElo)/2 BETWEEN 1600 AND 1799 THEN '1600–1799'
        WHEN (g.WhiteElo + g.BlackElo)/2 BETWEEN 1800 AND 2199 THEN '1800–2199'
        ELSE '2200 and above'
    END AS elo_category,

  COUNT(*) AS total_games,

  -- Victories where advantaged player actually won
  SUM(CASE 
        WHEN ft.advantaged_player = 'white' AND g.Result = 'White' THEN 1
        WHEN ft.advantaged_player = 'black' AND g.Result = 'Black' THEN 1
        ELSE 0
      END) AS victories,

  -- Draws
  SUM(CASE WHEN g.Result = 'Tie' THEN 1 ELSE 0 END) AS draws,

  -- Percentages
  ROUND(100.0 * 
        SUM(CASE 
              WHEN ft.advantaged_player = 'white' AND g.Result = 'White' THEN 1
			  WHEN ft.advantaged_player = 'black' AND g.Result = 'Black' THEN 1
			  ELSE 0
            END) / COUNT(*), 2) AS win_pct,

  ROUND(100.0 * SUM(CASE WHEN g.Result = 'Tie' THEN 1 ELSE 0 END) / COUNT(*), 2) AS draw_pct

FROM final_type ft
JOIN game g ON g.GameId = ft.GameId
WHERE 
  ft.advantaged_player IS NOT NULL
  AND g.WhiteElo IS NOT NULL
  AND g.BlackElo IS NOT NULL
  AND ft.endgame_type != 'empty_endings'

GROUP BY ft.endgame_type, ft.advantaged_player, elo_category
ORDER BY 
  ft.endgame_type,
  advantaged_player,
  CASE elo_category
    WHEN 'Under 1400' THEN 1
    WHEN '1400–1599' THEN 2
    WHEN '1600–1799' THEN 3
    WHEN '1800–2199' THEN 4
    WHEN '2200 and above' THEN 5
    ELSE 6
  END

