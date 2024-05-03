
def count_possible_games(games):
    sum_of_powers = 0

    for game in games:
        _,draws = game.split(': ')

        min_reqd = {'red':0, 'green':0, 'blue':0}
        for draw in draws.split('; '):
            for color in draw.split(', '):
                color_count,color = color.split(' ')
                min_reqd[color] = max(min_reqd[color],int(color_count))

        print(game, min_reqd)
        sum_of_powers += min_reqd['red'] * min_reqd['green'] * min_reqd['blue']

    return sum_of_powers


                


if __name__ == '__main__':

    test_data = [
        'Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green',
        'Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue',
        'Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red',
        'Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red',
        'Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green',
    ]

    print(count_possible_games(test_data))
