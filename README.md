# Pittsburgh HealthyRide Stations Tour

## Motivation
There are 51 HealthyRide stations in Pittsburgh which seems like a reasonable number to ride a bike between in a day. What would be the shortest tour stopping once at each station and ending at the initial station?

## Station Locations
The first step is finding the coordinates of each station. This was accomplished easily by running the following JavaScript code on [this webpage](https://iframe.nextbike.net/reservation/?lat=40.4313684&lng=-79.9805005&zoom=12&height=400&maponly=1&language=en):
```javascript
for (var i = 0; i < stations.length; i++) {
    if (stations[i].city_uid == 254) {
        console.log(stations[i].lat + "," + stations[i].lng);
    }
}
```
These coordinates can be found in `data/nodes.csv`.

## Mathematica
A [reasonable](https://en.wikipedia.org/wiki/Travelling_salesman_problem#Computing_a_solution) shortest tour can be trivially obtained via Mathematica using the following code:
```mathematica
positions = {GeoPosition[{40.445834,-80.008882}],...,GeoPosition[{40.462025604284,-79.968114495277}]}
tour = Last[FindShortestTour[positions]]
GeoGraphics[{Thick, Red, Line[positions[[tour]]]}]
```
This produces the following tour:
![Mathematica Tour](/results/mathematicaTour.png)
This tour has a travel time of 3 hours 37 minutes 19 seconds, traversing counterclockwise a distance of 29.5 miles.

## Google Maps
The problem with this solution is that it uses an oversimplified model of the world. This isn't a flight path; the roads and bridges are of significance. Fortunately, the Google Maps [Distance Matrix API](https://developers.google.com/maps/documentation/distance-matrix/start) provides accurate biking times between points. The code in `scripts/distances.py` queries Google Maps to obtain the biking distance and duration for every possible pair of stations (2550 combinations altogether). These can be found in `data/costs.csv`.

## Simulated Annealing
Coercing Mathematica to use these values as edge weights was beyond my skills, so instead I wrote this program which uses [simulated annealing](https://en.wikipedia.org/wiki/Simulated_annealing) to obtain the solution. Simulated annealing can't guarantee an optimal solution, but it does a pretty good job nonetheless. The result is difficult to visualize, so I wrote `scripts/mapper.py` to print a map. Here is the best tour I was able to find:
![Best Tour](/results/bestTour.png)
Total travel time is 3 hours 5 minutes 29 seconds, traversing counterclockwise a distance of 26.7 miles.

## Next Steps
All that remains is to perform this tour and track it via GPS.
