<script lang="ts">
	import { state } from '$lib/state';
	import { status } from '$lib/sse';
	import { fade } from 'svelte/transition';
	import { Chasing } from 'svelte-loading-spinners';
</script>

{#if (!$state.needsLogin && (!$state.fetched || !$state.connected || !$state.gotFilters)) || $state.criticalError}
	<div class="loader" out:fade={{ duration: 200 }}>
		{#if $status.error || $state.criticalError}
			<div class="error" role="alert">
				{$state.criticalError || $status.error}
				<button
					on:click={() => location.reload()}
					class="w-full mt-5 bg-red-400 hover:bg-red-600 text-white font-bold py-2 px-4 rounded"
					>Reload</button
				>
			</div>
		{:else}
			<Chasing size="32" color="#BFD7B5" />
		{/if}
	</div>
{/if}

<style lang="sass">
	.loader
		background: var(--background-color)
		display: flex
		position: absolute
		width: 100%
		height: 100%
		justify-content: center
		align-items: center
		z-index: 100

	.error
		background-color: var(--error)
		@apply rounded-md text-black p-4 whitespace-pre-line overflow-auto max-w-2xl max-h-64
</style>
