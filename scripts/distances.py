#!/usr/bin/python
import csv
import requests

def coordinates(location):
	return str(location['latitude']) + ',' + str(location['longitude'])

def requestCost(origin, destination):
	r = requests.get('https://maps.googleapis.com/maps/api/distancematrix/json?origins=' +
			coordinates(origin) + '&destinations=' + coordinates(destination) +
			'&mode=bicycling&key=<REDACTED>')
	for row in r.json()['rows']:
		for element in row['elements']:
			return (str(origin['id']) + ',' +
					str(destination['id']) + ',' +
					str(element['distance']['value']) + ',' +
					str(element['duration']['value']))

locations = []
with open('nodes.csv', 'r') as f:
	reader = csv.reader(f)
	for row in reader:
		locations.append({'id': int(row[0]), 'latitude': float(row[1]), 'longitude': float(row[2])})

for origin in locations[-2:]:
	for destination in locations:
		if (origin != destination):
			print(requestCost(origin, destination))
