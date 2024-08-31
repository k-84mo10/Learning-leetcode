class Solution
{
public:
    double maxProbability(int n, vector<vector<int>> &edges, vector<double> &succProb, int start_node, int end_node)
    {
        vector<double> nodeProbFromStart(n, 0.0);
        bool is_update = true;

        nodeProbFromStart[start_node] = 1.0;

        while (is_update)
        {
            is_update = false;

            for (int i = 0; i < edges.size(); i++)
            {
                int a = edges[i][0];
                int b = edges[i][1];
                double prob = succProb[i];

                if (nodeProbFromStart[a] < nodeProbFromStart[b] * prob)
                {
                    nodeProbFromStart[a] = nodeProbFromStart[b] * prob;
                    is_update = true;
                }

                if (nodeProbFromStart[b] < nodeProbFromStart[a] * prob)
                {
                    nodeProbFromStart[b] = nodeProbFromStart[a] * prob;
                    is_update = true;
                }
            }
        }
        return nodeProbFromStart[end_node];
    }
};