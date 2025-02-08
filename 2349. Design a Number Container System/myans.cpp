class NumberContainers {
    std::unordered_map<int, int> index2number;
    std::unordered_map<int, std::set<int>> number2index;

public:
    NumberContainers() {}
    
    void change(int index, int number) {
        if (index2number.find(index) != index2number.end()) {
            int currentNumber = index2number[index];
            number2index[currentNumber].erase(index);
            if (number2index[currentNumber].empty()) number2index.erase(currentNumber);
        }
        index2number[index] = number;
        number2index[number].insert(index);
    }
    
    int find(int number) {
        if (number2index.find(number) == number2index.end() || number2index[number].empty()) return -1;
        return *number2index[number].begin();
    }
};

/**
 * Your NumberContainers object will be instantiated and called as such:
 * NumberContainers* obj = new NumberContainers();
 * obj->change(index,number);
 * int param_2 = obj->find(number);
 */
