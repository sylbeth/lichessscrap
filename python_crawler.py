import chess.pgn

with open("lichess_db_standard_rated_2013-01.pgn", 'r') as pgn:
    counter = 0
    game = True
    while game is not None:
        game = chess.pgn.read_game(pgn)
        counter += 1
        if counter % 100 == 0:
            print(game)
            print(counter)
