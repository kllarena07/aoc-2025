#include <iostream>
#include <fstream>
#include <string>


using namespace std;

int main() {
    std::ifstream inputFile("input.txt");
    std::string line;
    int count = 0;
    int value = 50;
    while (std::getline(inputFile, line)) {
        cout << "line:" << line;
        if(line[0] == 'R'){
            value += stoi(line.substr(1));
        } 
        else {
            value -= stoi(line.substr(1));
        }
        cout << " | value:" << value;
        value %= 100;
        if(value < 0){
            value += 100;
        }
        cout << " | concatenated value: " << value << endl;
        if (value == 0){
            ++count;
        }
    }
    cout << count << endl;
}