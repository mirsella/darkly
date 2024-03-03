# Explaination

When we look at the cookies section, we can see one cookie named `i_am_admin` set to the value `68934a3e9455fa72420237eb05902327` (`false`) hashed with MD5 algorithm.
So let's change the value of the cookie to `true` (`b326b5062b2f0e69046810717534cb09`) and refresh the page to get the flag.

# Use case

The insecure direct object reference vulnerability can be exploited by an attacker to access to other profiles by exemple if the profiles are in the `?page=profile&userID=1` format, the attacker can change the `userID` to access to other profiles.

# Fix

- To know if someone is an admin or not, we should not use a cookie to store this information, but check it in the backend.
