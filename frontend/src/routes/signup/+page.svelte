<script lang="ts">
    import { enhance } from "$app/forms";
</script>
<div class="sm mx-auto w-96 mt-24">
<div class="card p-4">
    <strong>Create Account</strong>
        <form method="post" use:enhance>
            <div class="form-element">
                <label for="firstname">First Name
                <input type="text" name="firstname" id="firstname" class="input variant-form-material !rounded-none"/>
                </label>
            </div>
            <div class="form-element">
                <label for="lastname">Last Name
                <input type="text" name="lastname" id="lastname" class="input variant-form-material !rounded-none"/>
                </label>
            </div>
            <div class="form-element">
                <label for="username">Username
                <input type="text" name="username" id="username" class="input variant-form-material !rounded-none"/>
                </label>
            </div>
            <div class="form-element">
                <label for="email">Email
                <input type="email" name="email" id="email" class="input variant-form-material !rounded-none"/>
                </label>
            </div>
            <div class="form-element">
                <label for="password">Password
                <input type="password" name="password" id="password" class="input variant-form-material !rounded-none"/>
                </label>
            </div>
            <div class="form-element">
                <label for="confirmpassword">Confirm Password
                <input type="password" name="confirmpassword" id="confirmpassword" class="input variant-form-material !rounded-none"/>
                </label>
            </div>
            <button class="variant-ghost-success mt-4 py-0.5 px-4 w-full" >create account</button>
        </form>
    <div class="mt-4">
        <a class="anchor" href="/login">Back to Login</a>
    </div>
</div>
</div>


<style>
    .form-element{
        margin-top: 1rem;
    }
</style>
