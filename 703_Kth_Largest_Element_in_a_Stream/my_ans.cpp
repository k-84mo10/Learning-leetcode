class KthLargest {
    int kth;
    vector<int> array;

public:
    KthLargest(int k, vector<int>& nums) : kth(k), array(nums) {}

    int getRandomNumber (int p, int r) {
        static std::mt19937 gen(std::random_device{}());
        std::uniform_int_distribution<> distrib(p, r);
        return distrib(gen);
    }

    int randomized_partition(int p, int r) {
        int i = getRandomNumber(p, r);
        std::swap(array[i], array[r]);
        return partition(p, r);
    }

    int partition(int p, int r) {
        int pivot = array[r];
        int i = p - 1;
        for (int j = p; j < r; j++) {
            if (array[j] < pivot) {
                i++;
                std::swap(array[i], array[j]);
            }
        }
        i++;
        std::swap(array[i], array[r]);
        return i;
    }

    int randomized_Select(int p, int r, int i) {
        while (p != r) {
            int q = randomized_partition(p, r);
            int k = q - p + 1;
            if (i == k) {
                return array[q];
            } else if (i < k) {
                r = q - 1;
            } else {
                p = q + 1;
                i = i - k;
            }
        }
        return array[p];
    }
    
    int add(int val) {
        array.push_back(val);
        return randomized_Select(0, array.size()-1, array.size()-kth+1);
    }
};

/**
 * Your KthLargest object will be instantiated and called as such:
 * KthLargest* obj = new KthLargest(k, nums);
 * int param_1 = obj->add(val);
 */