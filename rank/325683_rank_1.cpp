// Zadanie: Ranking
// Autor: Radoslaw Dabkowski
// Nr indeksu: 325683

#include <iostream>
#include <unordered_map>
#include <vector>
#include <algorithm>

int main() {
    // Initialize and read data
    uint32_t n;
    std::cin >> n;

    std::vector<uint32_t> scores(n);
    for (uint32_t i = 0; i < n; i++)
        std::cin >> scores[i];

    // Make a copy of the scores
    std::vector<uint32_t> sorted_scores = scores;

    std::sort(sorted_scores.begin(), sorted_scores.end(), std::greater<uint32_t>());

    // Create a map that maps a score to its rank
    std::unordered_map<uint32_t, uint32_t> score_to_rank;

    // Set the rank of the first score
    uint32_t rank = 1;
    score_to_rank[sorted_scores[0]] = rank;

    // Iterate over the sorted scores
    for (size_t i = 1; i < n; i++) {
        // If the current score is different from the previous score
        // increment the rank
        if (sorted_scores[i] != sorted_scores[i - 1])
            rank++;

        // Set the rank of the current score
        score_to_rank[sorted_scores[i]] = rank;
    }

    // Print the output
    std::cout << score_to_rank[scores[0]];
    for (size_t i = 1; i < n; i++)
        std::cout << " " << score_to_rank[scores[i]];

    return 0;
}
