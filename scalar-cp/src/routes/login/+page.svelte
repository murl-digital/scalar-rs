<script lang="ts">
    import { goto } from "$app/navigation";
    import { base } from "$app/paths";
    import { state as appState } from "$lib/state.svelte";

    let email = $state();
    let password = $state();

    async function login(event: SubmitEvent) {
        event.preventDefault();

        let response = await fetch(`${base}/api/signin`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                email: email,
                password: password,
            }),
        });

        if (response.ok) {
            sessionStorage.setItem("token", await response.text());
            appState.authenticated = true;
            goto(`${base}/`);
        }
    }
</script>

<dialog open>
    <form onsubmit={login}>
        <label>
            Email
            <input name="email" type="email" bind:value={email} />
        </label>
        <label>
            Password
            <input name="password" type="password" bind:value={password} />
        </label>
        <button>Log in</button>
    </form>
</dialog>
