#include <iostream>
#include <fstream>
#include <string>

using namespace std;

int main() {
    ifstream inputFile("input.txt");
    string line;

    int heheheha = 0;
    int baha = 50;

    while (getline(inputFile, line)) {

        cout << "line:" << line;

        int david = stoi(line.substr(1));
        int osuly = baha;

        bool grr = false;

        if (line[0] == 'R') {
            int jussy = (osuly + david) % 100;
            grr = (jussy < osuly);
            baha = jussy;
        } else {
            int jussy = (osuly - david) % 100;
            if (jussy < 0) jussy += 100;
            grr = (jussy > osuly);
            baha = jussy;
        }

        if (grr) heheheha++;
        if (baha == 0) heheheha++;

        cout << " | value:" << baha
             << " | passes 0:" << grr
             << " | total so far: " << heheheha << endl;
    }

    cout << heheheha << endl;
}
