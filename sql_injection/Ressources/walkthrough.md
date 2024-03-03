# Explanation

When we visit the page `/?page=member`, we have a search bar that allows us to search for a member by their ID. But when we let it empty, we have the error: `You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near '' at line 1`, which means that the website probably uses directly the value of the `id` parameter in a SQL query, so we can try to perform a SQL injection.

To get all the table and column names, we can use the following payload:
```sql
1 OR 1=1 UNION SELECT table_name, column_name FROM information_schema.columns
```

So we can try to select some `users` columns and finaly, we found the `Commentaire` and `countersign` columns.

```sql
ID: 1 or 1=1 UNION SELECT Commentaire, countersign FROM users
```
```
First name: Decrypt this password -> then lower all the char. Sh256 on it and it's good !
Surname : 5ff9d0165b4f92b14994e5c685cdce28
```

The decryption of the password is `FortyTwo`, and after lowering all the characters and hashing it with SHA256, we get the following hash: `10a16d834f9b1e4068b25c4c46fe0284e99e44dceaf08098fc83925ba6310ff5`