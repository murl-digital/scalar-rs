<script lang="ts">
    import { goto } from "$app/navigation";
    import { base } from "$app/paths";
    import { state as appState } from "$lib/state.svelte";
    import { fly } from "svelte/transition";

    let email = $state();
    let password = $state();
    let element: HTMLDialogElement | undefined = $state();

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
            goto(`${base}/`, { invalidateAll: true });
        }
    }

    $effect(() => {
        if (element) {
            element.showModal();
        }
    });
</script>

<dialog
    bind:this={element}
    class="my-auto p-8 bg-dark text-gray border-1 rounded-sm shadow-lg shadow-black backdrop:backdrop-brightness-50 backdrop:backdrop-blur-sm transition-all"
>
    <form onsubmit={login} class="flex flex-col">
        <label class="flex flex-col">
            Email
            <input
                class="input-base"
                name="email"
                type="email"
                bind:value={email}
            />
        </label>
        <label class="flex flex-col">
            Password
            <input
                class="input-base"
                name="password"
                type="password"
                bind:value={password}
            />
        </label>
        <button>Log in</button>
    </form>
</dialog>
