/*
ZadanieKonkurs programistyczny zgromadził dużą liczbę uczestników, którzy zdobywali punkty w różnych konkurencjach. Każdy uczestnik zapisując się do konkursu otrzymywał kolejny numer startowy. Na koniec zawodów, powstała lista wyników.Napisz program, który na podstawie wyników układa ranking uczestników. Jeśli wystartowało 5 zawodników uzyskując wyniki:11 15 25 15 31Ich rankingi są następujące:5 3231Zwróć uwagę na remisy. Jeśli takowy wystąpił, ranking wszystkich miejsc dzielonych jest taki sam. Można powiedzieć, że ranking uczestnika to liczba o jeden większa niż liczba uczestników, którzy osiągnęli lepszy od niegowynik punktowy.WejścieProgram czyta dane ze standardowego wejścia. W pierwszej linii znajduje się pojedyncza liczba N oznaczająca liczbę uczestników (1 ≤N ≤ 10000 000). Następnie znajdują się wyniki  punktowe  kikolejnych zawodników (0 ≤ki≤ 100000 000, dla 1≤ i≤ N).WyjścieProgram powinien wypisać na standardowe wyjścielistę rankingówri(dla  1≤ i≤ N)  kolejnych zawodnikówoddzielone spacjami.Przykłady1.Wejście:511 15 25 15 31Wyjście:5 32312.Wejście:100 1 1 2 2 1 2 0 0 2Wyjście:8 5 5 1 1 5 1 8 8 1
3.Wejście:1060 1530 30 40 40 40 50 10 20Wyjście:19 6 6 3 3 3 2108
*/

use std::io;

fn main() {
    // Read input
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let scores: Vec<u32> = input
        .split_whitespace()
        .map(|x: &str| x.parse().unwrap())
        .collect();
    
    // Sort scores
    let sorted_scores:Vec<u32> = {
        let mut scores = scores.clone();
        scores.sort();
        scores.reverse();
        scores
    };
    
    // Create a hashmap of scores and their rank:
    // Score -> Rank
    let mut score_rank: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
    // Set the rank of the first score to 1
    score_rank.insert(sorted_scores[0], 1);
    // Set the rank of the rest of the scores
    for i in 1..n {
        if sorted_scores[i] != sorted_scores[i-1] {
            score_rank.insert(sorted_scores[i], (i+1) as u32);
        }
    }

    // Print the rank of each score
    print!("{}", score_rank.get(&scores[0]).unwrap());
    for i in 1..n {
        print!(" {}", score_rank.get(&scores[i]).unwrap());
    }
}