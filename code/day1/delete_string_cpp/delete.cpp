#include <string>
#include <iostream>

void do_something(std::string * s) {
    std::cout << *s << "\n";
}

int main() {
    auto s = new std::string("Hello World");
    delete s;
    do_something(s);
    return 0;
}