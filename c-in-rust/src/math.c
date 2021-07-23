struct Point {
    int* x;
    int* y;
};

long long slength(struct Point* p) {
    return (long long)(*p->x) * (*p->x) + (long long)(*p->y) * (*p->y);
}