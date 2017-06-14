#pragma once

#include <string>  // string
#include <map>     // map
#include <utility> // pair
#include <fstream> // ifstream

class CostMap {
public:
	CostMap(std::string filename);
	int distance(int origin, int destination) const;
	int duration(int origin, int destination) const;
private:
	std::map<std::pair<int, int>, std::pair<int, int>> _costs;
};
