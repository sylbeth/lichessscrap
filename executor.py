from subprocess import run

for i in range(2013,2019):
    for j in range(1,13):
        run([".\\lichess.exe", f'E:\\lichess\\lichess_db_standard_rated_{i}-{j:>02}.pgn.zst', "@sample.arg", "-vv", "-f"])
