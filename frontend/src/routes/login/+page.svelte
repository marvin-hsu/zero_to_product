<script lang="ts">
    import {Input, Loader, Notification} from '@svelteuidev/core';
    import {Person, Keyboard, Check, Cross2} from 'radix-icons-svelte';
    import {Button} from '@svelteuidev/core';

    let username = '';
    let password = '';

    const url = 'https://marvinhsu-zero-to-production.fly.dev/login'
    let promise: Promise<any>;
    const login = () => {
        promise = fetch(url, {
            method: "Post",
            body: JSON.stringify({
                "user_name": username,
                "password": password
            })
        })
            .then(response => response.json())
    };
</script>


<Input icon={Person} placeholder="username" bind:username/>

<Input icon="{Keyboard}" placeholder="password" type="password" bind:password/>

<Button color="indigo" on:click={login}>
    Login
</Button>


{#await promise}
    <Loader/>
{:then data}
    <Notification title='Teal notification' icon={Check} color='teal'>
        Login Success.
    </Notification>
{:catch error}
    <Notification icon={Cross2} color='red'>
        Login Failed.
    </Notification>
{/await}




