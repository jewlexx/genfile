#include <iostream>
#include <fstream>
#include <stdint.h>

using std::cout, std::ofstream;

int main(int argc, char **argv)
{
    uint8_t byte = 0;

    ofstream GeneratedFile("file.txt");

    for (int i = 0; i < 1024; i++)
        GeneratedFile << byte;

    GeneratedFile.close();

    return 0;
}
