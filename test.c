// int main(void) {
//     int a;  // Uninitialized declaration
//     int b = 10;  // Initialized declaration
//     int c = b + 5;  // Declaration with arithmetic expression
    
//     a = b * 2;  // Assignment with multiplication
    
//     // Nested expression with multiple operations
//     int d = (a + b) * (c - 3);
    
//     // Expression statement that doesn't return anything
//     a = b + (c * (d - 2));
    
//     // Chained assignments
//     int e  = a + b;
    
//     // Complex arithmetic with parentheses
//     int g = ((a + b) * (c - d)) / (e + 1);
    
//     // Multiple operations without explicit return
//     a = b + c;
//     b = a - g;
//     c = b * d;
//  *******NO RETURN SO IT WILL IMPLICITLY RETURN 0*****
//}

int main(void) {
    // Bitwise and arithmetic operations
    int a = 45;   // 00101101 in binary
    int b = 22;   // 00010110 in binary
    
    // Bitwise operations
    int c = a & b;  // Bitwise AND
    int d = a | b;  // Bitwise OR
    int e = a ^ b;  // Bitwise XOR
    
    // Shifts
    int f = a << 2;  // Left shift
    int g = b >> 1;  // Right shift
    
    // Complex arithmetic with precedence
    int h = (a + b) * (c - d) / (e + 1);
    
    // Logical operations
    int i = (a > b) && (c < d);
    int j = (a == 45) || (b != 22);
    
    // Final calculation to return
    int k = h + f - g + (i * 10) + (j * 5);
    
    return k; // Return 119 according to gcc and my compiler
}