#include <iostream>
#include <fstream>
#include <string>

using namespace std;

int main() {
    ifstream inputFile("input.txt");
    string line;

    int heheheha = 0;   // password
    int baha = 50;      // dial

    while (getline(inputFile, line)) {

        cout << "line:" << line;

        char peepee = line[0];
        int jussy = stoi(line.substr(1));

        for (int i = 0; i < jussy; i++) {

            if (baha == 0) heheheha++;

            if (peepee == 'L') {
                baha--;
                if (baha < 0) baha = 99;
            } else {
                baha++;
                if (baha > 99) baha = 0;
            }
        }

        cout << " | value:" << baha
             << " | total so far: " << heheheha << endl;
    }

    cout << heheheha << endl;
}
