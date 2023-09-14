#!/usr/bin/env python3

import random
import sys

while True:
    board = []
    for _ in range(3):
        row = sys.stdin.readline().strip().split(',')
        board.append(row)
    
    while True:
        x, y = random.randint(0, 2), random.randint(0, 2)
        if board[x][y] == "":
            print(f"{x},{y}")
            sys.stdout.flush()
            break
