<script lang="ts">
    let email = '';
    let name = '';
    const url = 'https://marvinhsu-zero-to-production.fly.dev/subscriptions'
    let promise: Promise<any>;
    const subscribe = () => {
        promise = fetch(url, {
            method: "Post",
            headers: {
                "Content-Type": "application/x-www-form-urlencoded"
            },
            body: `email=${email}&name=${name}`
        })
            .then(response => console.log(response))
    };
</script>

<div class="container-fluid h-custom">
    <div class="row justify-content-center">
        <div class="col-10 col-sm-8 col-md-6 col-lg-4 col-xl-4 col-xxl-3">
            <div class="mt-5 mb-3">
                <label for="email" class="form-label">Email address</label>
                <input type="email" class="form-control" id="email" placeholder="name@example.com" bind:value={email}>
            </div>
            <div class="mb-3">
                <label for="name" class="form-label">Name</label>
                <input type="text" class="form-control" id="name" bind:value={name}>
            </div>
            <div class="text-center mt-4 pt-2 mb-4">
                <button type="button" class="btn btn-primary btn-lg" on:click={subscribe}>
                    Submit
                </button>
            </div>
        </div>
    </div>
</div>
