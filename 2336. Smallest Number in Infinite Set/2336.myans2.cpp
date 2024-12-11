class SmallestInfiniteSet {
    std::set<int> s;
    int min;
public:
    SmallestInfiniteSet() {
        min = 1;
    }
    
    int popSmallest() {
        int smallest;
        if (s.empty()) {
            smallest = min;
            min++;
        } else {
           smallest = *s.begin();
           s.erase(smallest); 
        }
        return smallest;
    }
    
    void addBack(int num) {
        if (num < min && s.find(num) == s.end()) s.insert(num);
    }
};

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * SmallestInfiniteSet* obj = new SmallestInfiniteSet();
 * int param_1 = obj->popSmallest();
 * obj->addBack(num);
 */