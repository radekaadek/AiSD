/*
ZadanieKonkurs programistyczny zgromadził dużą liczbę uczestników, którzy zdobywali punkty w różnych konkurencjach. Każdy uczestnik zapisując się do konkursu otrzymywał kolejny numer startowy. Na koniec zawodów, powstała lista wyników.Napisz program, który na podstawie wyników układa ranking uczestników. Jeśli wystartowało 5 zawodników uzyskując wyniki:11 15 25 15 31Ich rankingi są następujące:5 3231Zwróć uwagę na remisy. Jeśli takowy wystąpił, ranking wszystkich miejsc dzielonych jest taki sam. Można powiedzieć, że ranking uczestnika to liczba o jeden większa niż liczba uczestników, którzy osiągnęli lepszy od niegowynik punktowy.WejścieProgram czyta dane ze standardowego wejścia. W pierwszej linii znajduje się pojedyncza liczba N oznaczająca liczbę uczestników (1 ≤N ≤ 10000 000). Następnie znajdują się wyniki  punktowe  kikolejnych zawodników (0 ≤ki≤ 100000 000, dla 1≤ i≤ N).WyjścieProgram powinien wypisać na standardowe wyjścielistę rankingówri(dla  1≤ i≤ N)  kolejnych zawodnikówoddzielone spacjami.Przykłady1.Wejście:511 15 25 15 31Wyjście:5 32312.Wejście:100 1 1 2 2 1 2 0 0 2Wyjście:8 5 5 1 1 5 1 8 8 1
3.Wejście:1060 1530 30 40 40 40 50 10 20Wyjście:19 6 6 3 3 3 2108
*/

use std::io;

fn solve() {
    // Read input
    let mut input_n: String = String::new();
    io::stdin().read_line(&mut input_n).ok().expect("First line not found");

    // Assert that first line is a number in range 1..10000000
    let n: usize = input_n.trim().parse().ok().expect("First line is not a number");

    // Assert that n is in range 1..10000000, if not, tell user
    if n == 0 || n > 10000000 {
        panic!("First line is not in range 1..10000000");
    }
    
    let mut number_input: String = String::new();

    // Read second line
    io::stdin().read_line(&mut number_input).ok().expect("Second line not found");
    let scores: Vec<u32> = {
        let mut scores: Vec<u32> = Vec::new();
        for score in number_input.trim().split_whitespace() {
            scores.push(score.parse().ok().expect(format!("{} is not a valid number", score).as_str()));
        }
        scores
    };

    // Assert that number of scores is equal to n
    if scores.len() != n {
        panic!("Number of scores is not equal to n");
    }

    // Sort scores
    let sorted_scores:Vec<u32> = {
        let mut sorted_scores: Vec<u32> = scores.clone();
        sorted_scores.sort();
        sorted_scores.reverse();
        sorted_scores
    };
    
    // Create a hashmap of scores and their rank:
    // Score -> Rank
    let mut score_rank: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
    score_rank.insert(sorted_scores[0], 1);
    for i in 1..sorted_scores.len() {
        if sorted_scores[i] != sorted_scores[i-1] {
            score_rank.insert(sorted_scores[i], i as u32 + 1);
        }
    }

    // Print output without a trailing space
    print!("{}", score_rank.get(&scores[0]).unwrap());
    for i in 1..scores.len() {
        print!(" {}", score_rank.get(&scores[i]).unwrap());
    }
}

fn main() {
    solve();
}