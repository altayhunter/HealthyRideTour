#include "tour.h"
#include <algorithm>  // shuffle
#include <random>     // random_device, uniform_int_distribution

using namespace std;

Tour::Tour(string datafile, string costsfile, double temperature, double alpha)
		: _costmap(costsfile), _annealer(temperature, alpha), _mt((random_device()())){
	readFromFile(datafile);
	shuffle(begin(_nodes), end(_nodes), _mt);
	_cost = candidateCost(_nodes);
}

void Tour::mutate() {
	uniform_int_distribution<> dist(0, _nodes.size() - 1);
	int first, second;
	do {
		first = dist(_mt);
		second = dist(_mt);
	} while (first == second);
	vector<Position> candidate = mutation(first, second);
	int diff = _cost - candidateCost(candidate);
	// Note: This short-circuit only allows temperature to drop when
	// cost increases. Consider removing the diff > 0 part.
	if (diff > 0 || _annealer.verdict(diff)) {
		_nodes = candidate;
		_cost -= diff;
	}
}

bool Tour::done() const {
	return _annealer.done();
}

int Tour::cost() const {
	return _cost;
}

void Tour::readFromFile(string filename) {
	ifstream in(filename);
	Position p;
	while (in >> p) {
		_nodes.push_back(p);
	}
}

vector<Position> Tour::mutation(int first, int second) const {
	vector<Position> mutant = _nodes;
	int cap = mutant.size();
	int distance = (second - first + cap) % cap;
	for (int i = 0; i < (distance + 1) / 2; i++) {
		swap(mutant[first], mutant[second]);
		first = incrementMod(first);
		second = decrementMod(second);
	}
	return mutant;
}

int Tour::candidateCost(const vector<Position>& candidate) const {
	int cost = _costmap.duration(candidate.back().id, candidate.front().id);
	for (auto iter = candidate.begin(); iter != prev(candidate.end()); ++iter) {
		cost += _costmap.duration(iter->id, next(iter)->id);
	}
	return cost;
}

int Tour::incrementMod(int index) const {
	return (index + 1) % _nodes.size();
}

int Tour::decrementMod(int index) const {
	int cap = _nodes.size();
	return (index - 1 + cap) % cap;
}

bool operator<(const Tour& lhs, const Tour& rhs) {
	return lhs.cost() < rhs.cost();
}

ostream& operator<<(ostream& out, const Tour& t) {
	for (auto node : t._nodes) {
		out << node << endl;
	}
	out << "Duration: " << t.cost() << " seconds" << endl;
	return out;
}
