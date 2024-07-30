class Solution
{
public:
    int numUniqueEmails(vector<string> &emails)
    {
        unordered_set<string> email_addresses;

        for (string &email : emails)
        {
            int i = 0;
            int email_size = email.size();
            string forwarded_email;
            while (i < email_size)
            {
                if (email[i] == '.')
                {
                    i++;
                    continue;
                }
                if (email[i] == '+')
                    break;
                if (email[i] == '@')
                    break;

                forwarded_email.push_back(email[i]);
                i++;
            }

            while (email[i] != '@')
                i++;

            while (i < email_size)
            {
                forwarded_email.push_back(email[i]);
                i++;
            }

            email_addresses.insert(forwarded_email);
        }

        return email_addresses.size();
    }
};