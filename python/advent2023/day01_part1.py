import sys

def parse_and_sum(dataset):
    tot = 0
    for s in dataset:
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
        '1abc2',
        'pqr3stu8vwx',
        'a1b2c3d4e5f',
        'treb7uchet',
    ]

    result = parse_and_sum(test_data)
    print(result)
