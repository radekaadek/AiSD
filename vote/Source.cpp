#include <iostream>

using namespace std;


int main() {
    int N, M;
    cin >> N >> M;
    int candidates[1001] = { 0 }; //Przypisanie wartosci 0 wszystkim komorkom tablicy

    int k;
    int win = 0; //co zrobic zeby to dzialalo
    for (int i = 0; i < M; i++) {
        cin >> k;
        candidates[k]++;
        
    }
    
    for (int i = 1; i < N + 1; i++) {
		if (candidates[k] > win) {
            win = candidates[k];
        } //obliczenie ilosci glosow oddanych na kanydata, gdzie kandydat jest reprezentowany przez indeks tablicy
    }


    for (int i = 1; i < N + 1; i++) {

        if (candidates[i] == win) {
            cout << i << " ";
        }
    }
    return 0;
}