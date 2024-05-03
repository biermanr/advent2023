import sys

def parse_and_sum(dataset):
    spelled_numbers = {
        'zero':'0',
        'one':'1',
        'two':'2',
        'three':'3',
        'four':'4',
        'five':'5',
        'six':'6',
        'seven':'7',
        'eight':'8',
        'nine':'9',
    }

    tot = 0
    for s in dataset:
        #first replace "threeight" with "three3threeight" for all
        #doing this so that we don't disturb "nested" spellings like this
        #if we did "threeight" -> "3ight" greedily, we'd lose eight
        for spelled,num in spelled_numbers.items():
            s = s.replace(spelled,spelled+num+spelled)

        first_num = None
        last_num = None

        for l in s:
            if not l in {'0','1','2','3','4','5','6','7','8','9'}:
                continue

            if not first_num:
                first_num = l
                last_num = l
            else:
                last_num = l

        num = int(first_num+last_num)
        tot += num


    return tot

    


if __name__ == '__main__':
    test_data = [
        'two1nine',
        'eightwothree',
        'abcone2threexyz',
        'xtwone3four',
        '4nineeightseven2',
        'zoneight234',
        '7pqrstsixteen',
    ]

    result = parse_and_sum(test_data)
    print(result)
