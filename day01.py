#!/usr/bin/env python3

from common import load_input

def part1(input: list[str]):
    position = 50
    zero_count = 0

    for line in input:
        direction = -1 if line[0] == 'L' else 1
        amount = int(line[1:]) * direction

        position = (position + amount) % 100

        if position == 0:
            zero_count += 1

    return zero_count

def part2(input: list[str]):
    position = 50
    zero_count = 0

    for line in input:
        starting_position = position
        #last_zero_count = zero_count
        direction = -1 if line[0] == 'L' else 1

        # This cannot be as hard as I'm making it.

        # The // operator doesn't behave quite the way it intuitively feels
        # like it should with negative numbers, so we need to leave the amount
        # positive until we're done with this divmod.
        amount = int(line[1:])
        full_rotations, remainder = divmod(amount, 100)

        # Every 100 units in the instruction is just a free zero pass
        # without having to do anything tricky.
        zero_count += full_rotations

        # And now we need to put the sign back on whatever's left over
        # and adjust our actual position.
        amount = remainder * direction
        position += amount

        # If this causes you to land on or overshoot 0 or 100, add one more to the count.
        # Note that the magnitude of amount will always be less than 100 at this point,
        # so if you started at 0, you cannot possibly pass 0 at this step.
        if starting_position != 0 and (position <= 0 or position >= 100):
            zero_count += 1
        
        # And, finally, normalize the position to actually be on the dial.
        position %= 100
        #print('At', position,', passed zero', zero_count - last_zero_count, 'times')

    return zero_count


def main():
    input = load_input('day01.input.txt')
    print(part1(input))
    print(part2(input))

if __name__ == '__main__':
    main()