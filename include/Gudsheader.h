#ifndef GUDSHEADER_H
#define GUDSHEADER_H
//Best practice
//To maximize the chance that missing includes will be flagged by compiler, order your #includes as follows:
//    The paired header file
//    Other headers from your project
//    3rd party library headers
//    Standard library headers
//The headers for each grouping should be sorted alphabetically (unless the documentation for a 3rd party library instructs you to do otherwise).
// Det å linke til relativ dir står at er bad practice så jeg skal finne ut av hvordan man ungår det

// File: myclass.h


// Sånn headers virker er faktisk bare trolling
// I SRC folderen må vi nå ha en Gudsheader.cpp som linker til denne for at alt dette skal virke

class MyClass {
public:
    // Constructor
    MyClass(int a, int b);

    // Member functions
    int add();
    int subtract();
    int multiply();
    double divide();

private:
    int operand1;
    int operand2;
};

#endif