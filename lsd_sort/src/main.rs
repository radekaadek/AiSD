/*

Zadanie 5.10. Sortowanie pozycyjne LSD
Zaimplementuj i zbadaj algorytmy sortowania pozycyjnego LSD. Jakie własności ma algorytm
sortowania pozycyjnego LSD? Porównaj złożoność czasową sortowania pozycyjnego LSD z
sortowaniem szybkim. Zaproponuj zastosowania algorytmu sortowania pozycyjnego LSD.
Przeprowadź testy dla danych wejściowych będących ciągami znaków (napisy) o różnych
charakterystykach – np.: ciągi znaków o stałej długości, podobnej długości, różniące się znacznie.
Dodatkowe kryterium: krótkie ciągi, długie, bardzo długie. Implementując algorytmy sortowania
pozycyjnego łatwo wpaść w pułapkę dużego kosztu operacji kopiowania danych. Aby tego uniknąć,
sortuj wskaźniki!

*/

// random
use rand::Rng;

// sort by Least Significant Bit
fn lsd_sort(arr: &mut Vec<u8>) {
    
    let arr_len = arr.len();

    for i in 0..8 as u8 {

        // count number of 0s and 1s
        let mut count: Vec<u8> = vec![0, 0];
        for j in 0..arr_len {
            count[((arr[j] >> i) & 1) as usize] += 1;
        }
        count[1] += count[0];

        // sort using count
        let mut aux: Vec<u8> = vec![0; arr_len];
        for j in (0..arr_len).rev() {
            // here we need to subtract 1 because count is 1-indexed
            aux[(count[((arr[j] >> i) & 1) as usize] - 1) as usize] = arr[j];
            count[((arr[j] >> i) & 1) as usize] -= 1;
        }
        for j in 0..arr_len {
            arr[j] = aux[j];
        }
    }
    
}


// some tests
fn main() {
    // generate random arrays and sort them
    let no_elements: usize = 100000000;
    let mut rng = rand::thread_rng();

    let mut arr: Vec<u8> = vec![0; no_elements];
    for i in 0..no_elements {
        arr[i] = rng.gen();
    }
    let mut arr2 = arr.clone();
    println!("Sorting {} elements using LSD...", no_elements);
    let timer = std::time::Instant::now();
    lsd_sort(&mut arr);
    println!("Time elapsed: {} ms", timer.elapsed().as_millis());
    println!("Sorting {} elements using sort()...", no_elements);
    let timer = std::time::Instant::now();
    arr2.sort();
    println!("Time elapsed: {} ms", timer.elapsed().as_millis());
}
