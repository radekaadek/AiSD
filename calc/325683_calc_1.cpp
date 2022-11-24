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

	// Wczytywanie danych
	while(std::cin.get(token)) {
		switch (token) {
		case '(':
		case ' ':
			break;
		// Operatory laduje na stos
		case '+':
		case '*':
		case '/':
		case '-':
			operator_stack.push(token);
			break;
		case ')':
		    // Zdejmuje operatory ze stosow i wykonuje dzialanie
			// Wynik wraca na stos
			value_2 = value_stack.top();
			value_stack.pop();
			value_1 = value_stack.top();
			value_stack.pop();
			operator_token = operator_stack.top();
			operator_stack.pop();
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
				// Sprawdzanie dzielenia przez 0
				if (!value_2) {
					std::cout << "NaN";
					exit(1);
				}
				value_stack.push(value_1 / value_2);
				break;
			}
			break;
		// Cyfry ladujemy na stos
		default:
			value_stack.push(token - '0');
			break;
		}
	}
	std::cout << value_stack.top() << std::endl;
	return 0;
}