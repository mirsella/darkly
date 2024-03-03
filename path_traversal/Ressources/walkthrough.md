# Explaination

When navigating on the website, we can see the variable `?page=<xxx>` in the URL. This an indicator that the website is possibly vulnerable to path traversal.
If we try to navigate to `?page=../`, the website display a troll message, but this is a good indicator that the website is vulnerable to path traversal.
Try after try, we can find the path to the `etc/passwd` file: `{BASE_URL}/index.php?page=../../../../../../../../../../etc/passwd` and get the flag.

# Use Case

We could use this vulnerability to get access to the source code of the website, or to get access to the database / configuration files.

# Fix

To fix this vulnerability, we should use a whitelist of allowed files, and check that the file is in the allowed directory before serving it.