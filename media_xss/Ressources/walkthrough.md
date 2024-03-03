# Explaination

Like in the redirection, the website use directly the src attribute of the img tag to display the image. This is a vulnerability because we can create an encoded script in base64 and use it as the src attribute of the img tag. The server will execute the script and we will be able to execute code on the server.
`{BASE_URL}/index.php?page=media&src=data:text/html;base64,PHNjcmlwdD5hbGVydCgiSSBoYWNrZWQgeW91Iik8c2NyaXB0Lz4=`
It give us the flag.

# Use Case

We could use this vulnerability to execute code on the server.

# Fix

To fix this breach, we could use the database id of the image to display it, and not the src attribute of the img tag. Also, we could check the file's magic number to ensure that the file is an image.
