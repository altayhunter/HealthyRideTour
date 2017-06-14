#pragma once
#include <random>     // mt19937, uniform_real_distribution

class Annealer {
public:
	Annealer(double temperature, double alpha);
	bool verdict(double cost);
	bool done() const;
private:
	double _temperature;
	double alpha;
	std::mt19937 _mt;
    std::uniform_real_distribution<double> _dist;
};
