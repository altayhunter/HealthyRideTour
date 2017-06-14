#pragma once
#include "position.h" // Position
#include "annealer.h" // Annealer
#include "costmap.h"  // CostMap
#include <vector>     // vector
#include <string>     // string
#include <random>     // mt19937

class Tour {
public:
	Tour(std::string datafile, std::string costsfile, double temperature, double alpha);
	void mutate();
	bool done() const;
	int cost() const;
private:
	void readFromFile(std::string filename);
	std::vector<Position> mutation(int first, int second) const;
	int candidateCost(const std::vector<Position>& candidate) const;
	int incrementMod(int index) const;
	int decrementMod(int index) const;

	CostMap _costmap;
	int _cost;
	Annealer _annealer;
	std::mt19937 _mt;
	std::vector<Position> _nodes;

	friend std::ostream& operator<<(std::ostream& out, const Tour& t);
};

bool operator<(const Tour& lhs, const Tour& rhs);
