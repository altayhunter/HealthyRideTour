#!/usr/bin/python
# Returns the total distance and duration of a tour, given the IDs in order.
# Example:
# $ calculator.py 1000 1001 1002 1003 1004 1005 1006 1007 1008 1042 1047 1048 1049 1046 1045 1044 1043 1036 1037 1038 1039 1041 1040 1035 1034 1033 1031 1032 1028 1027 1029 1026 1025 1024 1030 1023 1022 1021 1020 1019 1018 1061 1017 1016 1009 1010 1015 1014 1012 1013 1011 
# Distance: 43002 meters.
# Duration: 11129 seconds.

import csv
from sys import argv
from itertools import tee

def pairwise(iterable):
	a, b = tee(iterable)
	next(b, None)
	return zip(a, b)

file = '../data/costs.csv'
costs = {}

with open(file, 'r') as f:
	reader = csv.reader(f)
	for row in reader:
		costs[row[0], row[1]] = row[2:]

distance = 0
duration = 0
for a, b in pairwise(argv[1:] + argv[1:2]):
	distance += int(costs[a, b][0])
	duration += int(costs[a, b][1])

print("Distance:", distance, "meters.")
print("Duration:", duration, "seconds.")
