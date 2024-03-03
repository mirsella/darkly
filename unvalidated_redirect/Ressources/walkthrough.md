# Explaination

We can see 3 links for `twitter`, `facebook` and `instagram` in the footer of the page.
But they are not direct link, but `{BASE_URL}/index.php?page=redirect&site=facebook` for example.
But the `site` parameter is not validated, and we can change it to any value.

We can change the `site` parameter to `{BASE_URL}/index.php?page=redirect&site=https://www.youtube.com/watch?v=dQw4w9WgXcQ&pp=ygUIcmlja3JvbGw%3D` by example, and get the flag.

# Use case

The problem is that we can change the `site` parameter to any value, and it will redirect us to another website, and let people think that it's a link from the `BASE_URL` website. And could be used for phishing.

# Fix

We need to validate the `site` parameter, and check if it's a valid link from the website. And if not, we need to not redirect the user to the link.
Or we can put direct link, and not use the `redirect` page.