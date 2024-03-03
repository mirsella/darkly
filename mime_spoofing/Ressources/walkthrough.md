# Explaination

We can see an upload button to send images through the server. But there is a breach named mime_spoofing, which allows us to upload a file with a different extension than the one we want to upload. We can upload a file with a .sh extension, and the server will accept it as an image. Then we can execute the file by navigating to the file's URL.

```sh
echo "rm /etc/config.conf" > cute_file.sh && curl -s -X POST '{BASE_URL}/index.php?page=upload' -F "uploaded=@cute_file.sh;type=image/jpeg" -F "Upload=Upload" | grep flag && rm cute_file.sh
```

# Use Case

We could use this vulnerability to upload a shell script and execute it on the server.

# Fix

To fix this vulnerability, we should check the extension of the file in the backend, also, we should check the file's magic number to ensure that the file is an image.