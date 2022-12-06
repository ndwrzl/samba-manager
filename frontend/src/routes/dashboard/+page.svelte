<script lang="ts">
	import { logs } from '$lib/logStore';
	import Event from '$lib/Components/Event.svelte';
	import FilterPicker from '$lib/Components/FilterPicker.svelte';
	import { state } from '$lib/state';
	import { t } from '$lib/translations';
</script>

<svelte:head>
	<title>Samba Dashboard</title>
	<meta name="description" content="Samba Dashboard" />
</svelte:head>

{#if $state.connected && $state.loggedIn}
	<FilterPicker />
	<div class="mx-2 md:mx-16 xl:mx-24">
		<h1 class="text-lg font-bold">{$t('l.logs')}</h1>
		<div class="flex-auto overflow-y-auto">
			{#each $logs as event}
				<Event {event} />
			{/each}
		</div>
	</div>
{/if}
