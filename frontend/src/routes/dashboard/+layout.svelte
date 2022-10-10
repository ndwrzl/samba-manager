<script lang="ts">
	import { subscribe, unsubscribe } from '$lib/sse';
	import { state } from '$lib/state';
	import { getFilters, getLatestLogs } from '$lib/api';
	import Navbar from '$lib/Components/Navbar.svelte';
	import { browser } from '$app/environment';

	$: {
		if ($state.loggedIn && browser) {
			start();
		}
	}

	let started = false;
	async function start() {
		if (started) return;
		started = true;
		subscribe();
		if (await getLatestLogs()) {
			$state.fetched = true;
		}

		if (await getFilters()) {
			$state.gotFilters = true;
		}

		return unsubscribe;
	}
</script>

<Navbar />
<div class="flex flex-col flex-auto">
	<slot />
</div>
