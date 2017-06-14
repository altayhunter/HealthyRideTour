#include "tour.h"
#include <algorithm> // min

using namespace std;

constexpr int kTemperature = 1e7;
constexpr double kAlpha = 0.999;
constexpr char kDataFile[] = "data/nodes.csv";
constexpr char kCostsFile[] = "data/costs.csv";

int main() {
	Tour t(kDataFile, kCostsFile, kTemperature, kAlpha);
	Tour cheapest = t;
	while (!t.done()) {
		t.mutate();
		cheapest = min(cheapest, t);
	}
	cout << cheapest << endl;
	return 0;
}
