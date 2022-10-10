<script context="module">
	export const prerender = true;
</script>

<script lang="ts">
	import Loader from '$lib/Loader.svelte';
	import '../app.css';
	import { state, checkLogin } from '$lib/state';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';

	$: {
		if (!$state.needsLogin && $state.loggedIn && browser) {
			goto('/dashboard');
		}
	}

	if (browser) {
		onMount(() => {
			checkLogin();
		});
	}
</script>

<Loader />
<slot />
