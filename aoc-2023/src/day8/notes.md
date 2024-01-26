# day8part1
Camel Nodes
Maps - puzzle input
Left/Right instructions
Network of nodes
Orig -  AAA 
Dest -  ZZZ

This is a really hard puzzle!

*sample_input* 
LLR
AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
d = ['L','L','R']
- Start at Node 1: AAA 
state: Node(AAA), L(BBB), R(BBB), NEXT(L[d[0]]), PREV(None), PATTERN(LLR), PLACE(0)
move to L(node BBB) > find BBB > Node(BBB) > NEXT(d[1]:L) 
move to L(node AAA) > find AAA > next(d[2]:R) > 
move to R(node BBB) > next(if i > d.len then i = 0)


struct node = {name, L, R, `kkk
