#include <iostream>
using namespace std;

int main() {
    cout << "Start of the programm." << endl;
    cout << "Hello World!" << endl;
    for (int i = 0; i < 10; i++) {
        cout << i << endl;
        i+=i;
    }
    cout << "End of the programm." << endl;
    return 0;
}