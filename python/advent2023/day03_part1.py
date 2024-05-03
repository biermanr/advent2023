
def sum_part_numbers(diagram):
    allowed_digits = {'0','1','2','3','4','5','6','7','8','9'}

    def pos_is_symbol(i,j):
        if i < 0 or i >= len(diagram):
            return False
        if j < 0 or j >= len(diagram[0]):
            return False

        if diagram[i][j] in allowed_digits or diagram[i][j] == '.':
            return False

        return True

    def any_surrounding(i,j):
        return any([
            pos_is_symbol(i-1,j-1), #upper left corner
            pos_is_symbol(i-1,j  ), #directly above
            pos_is_symbol(i-1,j+1), #above right corner
            pos_is_symbol(i+1,j-1), #below left corner
            pos_is_symbol(i+1,j  ), #directly below
            pos_is_symbol(i+1,j+1), #below right corner
            pos_is_symbol(i  ,j-1), #left
            pos_is_symbol(i  ,j+1), #right
        ])

    parts_sum = 0
    for i in range(len(diagram)):
        num_ok = False
        digits = []
        for j,e in enumerate(diagram[i]):
            if e in allowed_digits:
                digits.append(e)
                num_ok = num_ok or any_surrounding(i,j)

            else:
                if len(digits) > 0 and num_ok:
                    parts_sum += int(''.join(digits))
                num_ok = False
                digits = []

        #need to check one more time at the end of this row
        if len(digits) > 0 and num_ok:
            parts_sum += int(''.join(digits))

    return parts_sum

if __name__ == '__main__':

    test_data = [
        '467..114..',
        '...*......',
        '..35..633.',
        '......#...',
        '617*......',
        '.....+.58.',
        '..592.....',
        '......755.',
        '...$.*....',
        '.664.598..',
    ]

    print(sum_part_numbers(test_data))
