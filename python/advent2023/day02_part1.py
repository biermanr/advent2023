
def count_possible_games(games):
    #how many RED/GREEN/BLUE cubes in the bag
    bag = {'red':12, 'green':13, 'blue':14}
    passing_game_sum = 0

    for game in games:
        game_num,draws = game.split(': ')
        game_num = int(game_num.split(' ')[1])
        game_ok = True

        for draw in draws.split('; '):
            for color in draw.split(', '):
                color_count,color = color.split(' ')
                color_count = int(color_count)
                if color_count > bag[color]:
                    game_ok = False

        if game_ok:
            passing_game_sum += game_num

    return passing_game_sum


                


if __name__ == '__main__':

    test_data = [
        'Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green',
        'Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue',
        'Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red',
        'Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red',
        'Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green',
    ]

    print(count_possible_games(test_data))
