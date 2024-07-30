class Solution
{
public:
    int numUniqueEmails(vector<string> &emails)
    {
        unordered_map<string, bool> email_adress_list;
        int count = 0;

        for (string email : emails)
        {
            string forwarded_adress;
            bool is_local_name = true;
            int i = 0;
            int email_size = email.size();
            while (i < email_size)
            {
                if (is_local_name)
                {
                    if (email[i] == '.')
                    {
                        i++;
                        continue;
                    }
                    if (email[i] == '+')
                    {
                        while (email[i] != '@')
                        {
                            i++;
                        }
                    }
                    if (email[i] == '@')
                        is_local_name = false;
                }
                forwarded_adress.push_back(email[i]);
                i++;
            }

            if (email_adress_list.find(forwarded_adress) == email_adress_list.end())
            {
                count += 1;
                email_adress_list[forwarded_adress] = true;
            }
        }

        return count;
    }
};