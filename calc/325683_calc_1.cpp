// Zadanie: Kalkulator
// Autor: Radoslaw Dabkowski
// Nr indeksu: 325683

// Przepraszam ze plik jest zle nazwany ale teams nie pozwala mi go nazwac poprawnie

#include <iostream>
#include <stack>


int main() {
	// Deklaracja zmiennych
	std::stack<int> value_stack;
	std::stack<char> operator_stack;
	char token, operator_token;
	int value_1, value_2; // Wartosci zdejmowane ze stosow
	uint64_t bracket_count = 0;
	// Wczytywanie danych
	while(std::cin.get(token)) {
		switch (token) {
		case '(':
			bracket_count++;
		case ' ':
			break;
		case ')':
			bracket_count--;
			value_2 = value_stack.top();
			value_stack.pop();
			value_1 = value_stack.top();
			value_stack.pop();
			operator_token = operator_stack.top();
			operator_stack.pop();
			// Wykonujemy operacje na zdjetych wartosciach
			// Wynik wraca na stos
			switch (operator_token) {
			case '+':
				value_stack.push(value_1 + value_2);
				break;
			case '-':
				value_stack.push(value_1 - value_2);
				break;
			case '*':
				value_stack.push(value_1 * value_2);
				break;
			case '/':
				if (!value_2) {
					std::cout << "NaN";
					exit(1);
				}
				value_stack.push(value_1 / value_2);
				break;
			}
			break;
		// Operatory i cyfry ladujemy na stos
		case '+':
		case '*':
		case '/':
			operator_stack.push(token);
			break;
		case '-':
			operator_stack.push('-');
			break;
		default:
			value_stack.push(token - '0');
			break;
		}
	}
	// Wypisujemy wynik
	std::cout << value_stack.top() << std::endl;
	return 0;
}