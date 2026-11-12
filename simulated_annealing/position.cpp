#include "position.h"

using namespace std;

istream& operator>>(istream& in, Position& p) {
	char comma;
	in >> p.id >> comma >> p.latitude >> comma >> p.longitude;
	return in;
}

ostream& operator<<(ostream& out, const Position& p) {
	out << p.id << " (" << p.latitude << "," << p.longitude << ")";
	return out;
}
