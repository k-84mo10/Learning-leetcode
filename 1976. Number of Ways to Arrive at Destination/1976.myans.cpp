#include <vector>
#include <queue>
#include <climits>
using namespace std;

class Solution {
public:
    vector<vector<pair<int, int>>> build_graph(int n, vector<vector<int>>& roads) {
        vector<vector<pair<int, int>>> graph(n);

        for (auto& road : roads) {
            int u = road[0];
            int v = road[1];
            int cost = road[2];

            graph[u].push_back({v, cost});
            graph[v].push_back({u, cost});
        }

        return graph;
    }

    int countPaths(int n, vector<vector<int>>& roads) {
        const long long MOD = 1e9 + 7;
        auto graph = build_graph(n, roads);

        vector<long long> min_dist(n, LLONG_MAX);  // long long に変更
        vector<long long> ways(n, 0);

        priority_queue<pair<long long, int>, vector<pair<long long, int>>, greater<>> pq;

        min_dist[0] = 0;
        ways[0] = 1;
        pq.push({0, 0});

        while (!pq.empty()) {
            auto [curr_dist, u] = pq.top();
            pq.pop();

            if (curr_dist > min_dist[u]) continue;

            for (auto& [v, cost] : graph[u]) {
                long long new_dist = curr_dist + cost;
                if (new_dist < min_dist[v]) {
                    min_dist[v] = new_dist;
                    ways[v] = ways[u];
                    pq.push({new_dist, v});
                } else if (new_dist == min_dist[v]) {
                    ways[v] = (ways[v] + ways[u]) % MOD;
                }
            }
        }

        return static_cast<int>(ways[n - 1]);
    }
};
