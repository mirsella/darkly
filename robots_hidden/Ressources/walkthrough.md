testing /robots.txt gives allows us the see the config file for web crawler. one of the entry is disallowing `/.hidden`. accessing it we can see a labyrinth of directories and sometimes README file with unhelpful messages.
we can try writing a script that will scrap all the links and try to find a flag.
running the script show a flag in one of the README.

# use case

thoses directories are not meant to be public, and can divulge sensitive information likes configuration files.

# fix

the configuration of the web server should not allow to access thoses directories.
