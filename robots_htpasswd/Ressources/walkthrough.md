navigating to /robots.txt we can see the config file for web crawler.

```
User-agent: *
Disallow: /whatever
Disallow: /.hidden
```

which points us to /whatever where we can find a public htpasswd file. this is a file that contains usernames and passwords for basic authentication. putting the hash for the user `root` into google we can see it's just simple md5 hash, and decoding it gives us `qwerty123@`.
i tried in the main login page but it didn't work, so i tried in the popular `/admin` page and it worked.

# use case

we can use this to connect ourself as admin, and access moderation and/or administrations features.

# fix

the htpasswd file should not be public, and even less encrypted using md5.
