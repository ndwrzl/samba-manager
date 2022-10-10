<script lang="ts">
	import type { Log } from '$lib/logStore';
	import { slide } from 'svelte/transition';
	import { formatDistanceToNow, fromUnixTime } from 'date-fns';

	const format = (x: number) => formatDistanceToNow(fromUnixTime(x));
	export let event: Log;
</script>

<div class="event flex flex-col overflow-auto" transition:slide|local>
	<div>
		<div class="action">
			<span class="decoration-teal-400 underline decoration-2">
				{event.action.replaceAll('_', ' ')}
			</span>
			{#if event.data}
				{#each event.data.split('|') as tag}
					<div class="tag">
						{tag}
					</div>
				{/each}
			{/if}
		</div>

		<div class="uppercase">
			<span class="text-sky-300">
				{event.client_name}
			</span>
			<span class="text-neutral-300">
				{event.client_ip}
			</span>
		</div>
	</div>
	<div class="flex flex-row flex-wrap items-center">
		{event.path}
		{#if event.path2}
			<svg xmlns="http://www.w3.org/2000/svg" xml:space="preserve" viewBox="0 0 490 490">
				<path d="m240.112 0 241.749 245.004L240.112 490H8.139L250.29 245.004 8.139 0z" />
			</svg>

			{event.path2}
		{/if}
	</div>
	<div class="date">
		{format(event.date)} ago
		<span class="ml-auto">{fromUnixTime(event.date).toLocaleString()}</span>
	</div>
</div>

<style lang="scss">
	.event {
		background-color: var(--secondary);
		border-left: 2px var(--accent) solid;
		@apply w-full p-2 m-2 rounded-sm;
	}

	svg {
		height: 14px;
		stroke: white;
		fill: white;
		@apply mx-1;
	}

	.date {
		@apply text-xs w-full text-sky-200;
	}

	.action {
		@apply capitalize;
	}
	.tag {
		@apply text-xs mx-1 inline-flex items-center font-bold uppercase px-2 bg-blue-200 text-blue-700 rounded-full;
	}
</style>
