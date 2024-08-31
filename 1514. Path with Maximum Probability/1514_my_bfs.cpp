class Solution
{
public:
    double maxProbability(int n, vector<vector<int>> &edges, vector<double> &succProb, int start_node, int end_node)
    {
        vector<vector<pair<int, double>>> graph(n);
        for (int i = 0; i < edges.size(); i++)
        {
            graph[edges[i][0]].emplace_back(edges[i][1], succProb[i]);
            graph[edges[i][1]].emplace_back(edges[i][0], succProb[i]);
        }

        vector<double> nodeProbabilityFromStart(n, 0);
        priority_queue<pair<double, int>> pq;

        nodeProbabilityFromStart[start_node] = 1.0;
        pq.emplace(1.0, start_node);

        while (!pq.empty())
        {
            double current_prob = pq.top().first;
            int current_node = pq.top().second;
            pq.pop();

            if (current_node == end_node)
                return current_prob;

            for (auto &neighbor : graph[current_node])
            {
                int neighbor_node = neighbor.first;
                double neighbor_prob = neighbor.second;
                if (neighbor_prob == 0.0)
                    continue;
                double new_prob = neighbor_prob * current_prob;
                if (new_prob > nodeProbabilityFromStart[neighbor_node])
                {
                    nodeProbabilityFromStart[neighbor_node] = new_prob;
                    pq.emplace(new_prob, neighbor_node);
                }
            }
        }

        return 0.0;
    }
};
