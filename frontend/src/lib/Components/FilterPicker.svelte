<script lang="ts">
	import { filters as filterData } from '$lib/logStore';
	import { filters } from '$lib/filterState';
	import { getLog } from '$lib/api';
	import { onMount, tick } from 'svelte';
	import { state } from '$lib/state';

	const ToCheck = ['share_name', 'client_name', 'client_ip', 'actions'] as const;
	$: {
		for (const name of ToCheck) {
			if ($filters[name] !== -1 && !$filterData[name][$filters[name]]) {
				$filters[name] = -1;
			}
		}
	}

	onMount(async () => {
		await tick();
		filters.subscribe(async (conf) => {
			console.debug('filter update');
			await getLog({
				share_name: $filterData.share_name[conf.share_name],
				client_name: $filterData.client_name[conf.client_name],
				client_ip: $filterData.client_ip[conf.client_ip],
				actions: $filterData.actions[conf.actions],
				before: conf.before,
				after: conf.after,
				search: conf.search
			}).then((logs) => {
				// TODO: update filters
				if (!$state.fetched) $state.fetched = true;
			});
		});
	});
</script>

<div class="filters flex justify-center my-5 flex-wrap">
	<select bind:value={$filters.share_name}>
		<option value={-1}>Share name</option>
		{#each $filterData.share_name as filter, i}
			<option value={i}>{filter}</option>
		{/each}
	</select>

	<select bind:value={$filters.client_name}>
		<option value={-1}>Client name</option>
		{#each $filterData.client_name as filter, i}
			<option value={i}>{filter}</option>
		{/each}
	</select>

	<select bind:value={$filters.client_ip}>
		<option value={-1}>Client IP</option>
		{#each $filterData.client_ip as filter, i}
			<option value={i}>{filter}</option>
		{/each}
	</select>

	<select bind:value={$filters.actions}>
		<option value={-1}>Action</option>
		{#each $filterData.actions as filter, i}
			<option value={i}>{filter}</option>
		{/each}
	</select>

	<input type="text" placeholder="Search" bind:value={$filters.search} />
</div>

<style lang="sass">
	.filters
		select
			@apply m-2 w-64 py-3 px-4 text-base rounded-lg border bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500
		input
			@apply m-2 w-64 py-3 px-4 text-base rounded-lg border bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500
</style>
