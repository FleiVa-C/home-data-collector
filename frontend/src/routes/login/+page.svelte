<head>
    <title>Sign In</title>
</head>

<div class="sm mx-auto w-96 mt-24">
    <div class="card p-4">
        <strong>Login</strong>
            <form method="POST">
                <div class="form-element">
                    <label>Email
                    <input type="email" class="input variant-form-material !rounded-none" placeholder="email"/>
                    </label>
                </div>

                <div class="form-element">
                    <label>Password
                    <input type="password" class="input variant-form-material !rounded-none" placeholder="password"/>
                    </label>
                </div>
                <button class="variant-ghost-success mt-4 py-0.5 px-4 w-full" type="submit">login</button>
            </form>
        <div class="flex space-x-8 mt-4">
            <p> No Account ?</p>
            <a class="anchor" href="/signup">Signup</a>
        </div>
    </div>
</div>

<style>
    .form-element{
        margin-top: 1rem;
    }
</style>
