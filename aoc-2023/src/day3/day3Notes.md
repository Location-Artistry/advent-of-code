467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
- Any number adjacent to symbol is a part number: *+#$
1. (y: line index, x: Vector of chars(c[0],c[1]...)
2. Iterate through each Line and search for symbols
3. Symbol found ex: (3,1)
4. Check all of these coords: 
    - (2,0),(3,0),(4,0)
    - (2,1),(3,1),(4,1)
    - (2,2),(3,2),(4,2)

Alternative:
1. (y: line index, x: Vector of chars(c[0],c[1]...)
2. Iterate through chars until number is found
    - Create new part_num_str
    - IF next char = num then add to part_num_str
    - IF not num THEN check for adjacent symboll
3. Iterate through part_num_str chars
4. For each number check if adjacent to symbol
5. IF adjacent then add number to part_list_vec
6. IF NOT THEN goto next char in part_num_str 
7. IF no more number chars then find next part_num_str
8. IF no more numbers on line then goto next line 
9. IF no more lines then sum part_list_vec

---
x = 0
4 - part_num_str.len = 0
4 is_numeric => true => add to part_num_str
x = 1
6 - part_num_str.len = 1
6 is_numeric => true => add to part_num_str
x = 2
7 - part_num_str.len = 2
7 is_numeric => true => add to part_num_str
x = 3
. - part_num_str.len = 3
. is_numeric => false & len > 0 then begin_symbol_search
    -  

