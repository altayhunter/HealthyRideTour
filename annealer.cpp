#include "annealer.h"

using namespace std;

Annealer::Annealer(double temperature, double alpha) :
		_temperature(temperature), alpha(alpha), _mt((random_device()())), _dist(0, 1) {
}

bool Annealer::done() const {
	return _temperature < 1e-6;
}

bool Annealer::verdict(double cost) {
	double rand = _dist(_mt);
	bool result = exp(cost / _temperature) > rand;
	_temperature *= alpha;
	return result;
}
