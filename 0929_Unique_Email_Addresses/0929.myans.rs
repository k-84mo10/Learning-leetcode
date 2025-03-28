use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique_email_set = HashSet::new();
        for email in emails {
            let names:Vec<&str> = email.split('@').collect();
            let local_name:Vec<&str> = names[0].split('+').collect();
            let validated_local_name = local_name[0].replace(".", "");
            let validated_email = validated_local_name + "@" + names[1];
            unique_email_set.insert(validated_email);
        }
        unique_email_set.len() as i32
    }
}