class SmallestInfiniteSet {
    std::set<int> s;
public:
    SmallestInfiniteSet() {
        for (int i = 1; i <= 1000; i++) s.insert(i);
    }
    
    int popSmallest() {
        if (s.empty()) return -1;
        int num = *s.begin();
        s.erase(num);
        return num;
    }
    
    void addBack(int num) {
        if (s.find(num) == s.end()) s.insert(num);
    }
};

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * SmallestInfiniteSet* obj = new SmallestInfiniteSet();
 * int param_1 = obj->popSmallest();
 * obj->addBack(num);
 */