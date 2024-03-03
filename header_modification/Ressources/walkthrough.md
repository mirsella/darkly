# Explaination

On the page `/?page=b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f`, we have a beautiful music, in the source code we can see: `You must come from : "https://www.nsa.gov/".` and `Let's use this browser : "ft_bornToSec". It will help you a lot.`. So let's make a curl request with the user-agent `ft_bornToSec` and the referer `https://www.nsa.gov/`.

```sh
curl -s "{BASE_URL}/?page=b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f" -A 'ft_bornToSec' -e "https://www.nsa.gov/" | grep flag
```

It returns:

```html
<center><h2 style="margin-top:50px;"> The flag is : f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188</h2><br/><img src="images/win.png" alt="" width=200px height=200px></center> <audio id="best_music_ever" src="audio/music.mp3"preload="true" loop="loop" autoplay="autoplay">
```

# Use case

It is not really a security issue, but it can be used as an extra protection. If the user-agent or the referer is not the good one, we can return an error page or a 404 page. It can be used to protect against web crawlers or to protect a page from being accessed by a direct link.

# Fix

Maybe we could add a token in the header to make sure that the user is coming from the right page. But it is not really a security issue, so it is not really necessary to fix it.
