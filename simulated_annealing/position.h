#pragma once
#include <iostream> // istream, ostream

struct Position {
	int id;
	double latitude;
	double longitude;
};

std::istream& operator>>(std::istream& in, Position& p);
std::ostream& operator<<(std::ostream& out, const Position& p);
