# Explaination

Arriving on the page, we are greeted with a simple form asking for a name and a message, so it looks like a XSS attack (that means we can inject some javascript code in the message field and it will be executed by the server).

But it doesn't work, so after some tests, I put in my name "script" and it gives me a flag, it is not an XSS attack but a simple string, so I don't really understand the point of this page.

# Use case

XSS attacks are very dangerous, they can be used to steal cookies, redirect the user to a malicious website, or even steal the user's session. So it is important to sanitize the user's input and escape the special characters.

# Fix

The server should escape the special characters and sanitize the user's input, so it can't be used to execute javascript code.