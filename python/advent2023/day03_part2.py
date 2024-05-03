
def sum_part_numbers(diagram):
    allowed_digits = {'0','1','2','3','4','5','6','7','8','9'}

    def is_gear(i,j):
        if i < 0 or i >= len(diagram):
            return False
        if j < 0 or j >= len(diagram[0]):
            return False

        return diagram[i][j] == '*'

    def surrounding_gears(i,j):
        gears = set()
        positions = [
            (i-1,j-1), #upper left corner
            (i-1,j  ), #directly above
            (i-1,j+1), #above right corner
            (i+1,j-1), #below left corner
            (i+1,j  ), #directly below
            (i+1,j+1), #below right corner
            (i  ,j-1), #left
            (i  ,j+1), #right
        ]

        for pos in positions:
            if is_gear(*pos):
                gears.add(pos)

        return gears

    gears = {}
    for i in range(len(diagram)):
        adj_gears = set()
        digits = []
        num_ok = False
        for j,e in enumerate(diagram[i]):
            if e in allowed_digits:
                digits.append(e)
                adj_gears = adj_gears.union(surrounding_gears(i,j))

            else:
                if len(digits) > 0:
                    for gear in adj_gears:
                        if gear not in gears:
                            gears[gear] = []
                        gears[gear].append(int(''.join(digits)))
                adj_gears = set()
                digits = []

        #need to check one more time at the end of this row
        if len(digits) > 0:
            for gear in adj_gears:
                if gear not in gears:
                    gears[gear] = []
                gears[gear].append(int(''.join(digits)))

    #Determine which gears have exactly two adj numbers
    gear_sum = 0
    for gear,nums in gears.items():
        if len(nums) == 2:
            gear_sum += nums[0]*nums[1]

    return gear_sum

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
