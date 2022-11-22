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
	char token;
	// Wczytywanie danych
	while (std::cin >> token) {
		if (token == '(') {
			// nic nie robimy
			continue;
		}
		else if (token == ')') {
			// Zdejmujemy wartosci i operator ze stosow
			int value2 = value_stack.top();
			value_stack.pop();
			int value1 = value_stack.top();
			value_stack.pop();
			char op = operator_stack.top();
			operator_stack.pop();
			// Wykonujemy operacje
			int result;
			switch (op) {
				case '+':
					result = value1 + value2;
					break;
				case '-':
					result = value1 - value2;
					break;
				case '*':
					result = value1 * value2;
					break;
				case '/':
					result = value1 / value2;
					break;
				default:
					std::cout << "Nieznany operator: " << op << std::endl;
					return 1;
			}
			// Wynik wraca na stos
			value_stack.push(result);
		}
		// Operatory i cyfry ladujemy na stos
		else if (token == '+' || token == '-' || token == '*' || token == '/') {
			operator_stack.push(token);
		}
		else {
			value_stack.push(token - '0');
		}
	}
}