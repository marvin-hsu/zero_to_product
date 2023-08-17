<script lang="ts">
    let username = '';
    let password = '';
    const url = 'https://marvinhsu-zero-to-production.fly.dev/login'
    let promise: Promise<any>;
    const login = () => {
        promise = fetch(url, {
            method: "Post",
            headers: {
                "Content-Type": "application/json"
            },
            credentials: 'include',
            body: JSON.stringify({
                "user_name": username,
                "password": password
            })
        })
            .then(response => console.log(response))
    };
</script>

<section class="vh-100">
    <div class="container-fluid h-custom">
        <div class="row d-flex justify-content-center align-items-center h-100">
            <div class="col-md-9 col-lg-6 col-xl-5">
                <img src="https://mdbcdn.b-cdn.net/img/Photos/new-templates/bootstrap-login-form/draw2.webp"
                     class="img-fluid" alt="Sample image">
            </div>
            <div class="col-md-8 col-lg-6 col-xl-4 offset-xl-1">
                <form>
                    <!-- Email input -->
                    <div class="form-outline mb-4">
                        <label class="form-label" for="form3Example3">Username</label>
                        <input type="text" id="form3Example3" class="form-control form-control-lg"
                               placeholder="Enter username" bind:value={username}/>
                    </div>

                    <!-- Password input -->
                    <div class="form-outline mb-3">
                        <label class="form-label" for="form3Example4">Password</label>
                        <input type="password" id="form3Example4" class="form-control form-control-lg"
                               placeholder="Enter password" bind:value={password}/>
                    </div>

                    <div class="text-center text-lg-start mt-4 pt-2">
                        <button type="button" class="btn btn-primary btn-lg"
                                style="padding-left: 2.5rem; padding-right: 2.5rem;"
                                on:click={login}>
                            Login
                        </button>
                    </div>
                    {#await promise}
                    {:then data}
                    {:catch error}
                    {/await}

                </form>
            </div>
        </div>
    </div>

</section>


