int add(int a, int b) {
    return a + b;
}

int mul(int x, int y) {
    return x * y;
}

int main() {
    int r1 = add(1, 2);
    int r2 = mul(r1, 3);
    return r2;
}
