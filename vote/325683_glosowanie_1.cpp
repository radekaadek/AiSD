// Zadanie: Glosowanie
// Autor: Radoslaw Dabkowski
// Nr indeksu: 325683

#include <iostream>

int main() {

	// Wczytanie danych wejsciowych i zainicjowanie zmiennych
	uint16_t N;
	uint32_t M;
	std::cin >> N >> M;
	uint32_t* candidates = new uint32_t[N + 1];
	for (uint16_t i = 0; i <= N; i++) {
		candidates[i] = 0;
	}

	// Zliczanie glosow na kandydatow
	uint16_t voter;
	for (uint32_t i = 0; i < M; i++) {
		std::cin >> voter;
		candidates[voter]++;
	}

	// Wyznaczenie najwiekszej ilosci głosow
	uint32_t max_votes = 0;
	for (uint16_t i = 1; i <= N; i++) {
		if (candidates[i] > max_votes) {
			max_votes = candidates[i];
		}
	}
	
	// Wypisanie wyniku
	for (uint16_t i = 1; i <= N; i++) {
		if (candidates[i] == max_votes) {
			std::cout << i << " ";
		}
	}

	delete[] candidates;

	return 0;

}
