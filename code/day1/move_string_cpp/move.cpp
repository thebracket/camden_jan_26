#include <string>
#include <iostream>

void do_something(std::string s) {
    std::cout << s << "\n";
}

int main() {
    auto s = std::string("Hello World");
    do_something(std::move(s));
    std::cout << s << "\n";
    return 0;
}
