#include "costmap.h"
#include <fstream>   // ifstream

using namespace std;

CostMap::CostMap(string filename) {
	ifstream in(filename);
	int origin;
	int destination;
	int distance;
	int duration;
	char comma;
	while (in >> origin >> comma >> destination >> comma >> distance >> comma >> duration) {
		_costs[make_pair(origin, destination)] = make_pair(distance, duration);
	}
}

int CostMap::distance(int origin, int destination) const {
	return _costs.at(make_pair(origin, destination)).first;
}

int CostMap::duration(int origin, int destination) const {
	return _costs.at(make_pair(origin, destination)).second;
}
