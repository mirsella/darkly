# Explanation

```html
<form action="#" method="POST">
	<input type="hidden" name="mail" value="webmaster@borntosec.com" maxlength="15">
	<input type="submit" name="Submit" value="Submit">
</form>
```

On the `/?page=recover` page, we can see that the email is in a hidden input field. So we can change the value of the email and submit the form, to get the flag.

# Use case

The attacker can change the email to his own and receive the password reset link.
Or he could spam or mailbomb the webmaster's email address.

# Fix

- The email should not be in a hidden input field. It should be in a text input field and send the email directly to the user's email address.
- If the webmaster's email has to receive the email, it should be stored in the backend and not in the frontend.
