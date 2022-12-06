<script lang="ts">
	import { locales, locale, t } from '$lib/translations';
	import { logout } from '$lib/api';
</script>

<nav>
	<p>SAMBA</p>
	<div class="ml-auto">
		<select
			bind:value={$locale}
			class="ml-auto self-end w-32 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 mb-2"
		>
			{#each $locales as value}
				<option {value}>{$t(`lang.${value}`)}</option>
			{/each}
		</select>
		<button on:click={logout}>{$t('common.logout')}</button>
	</div>
</nav>

<style lang="scss">
	nav {
		@apply flex items-center w-full p-2;
		background-color: var(--secondary);

		p {
			@apply h-min;
		}

		button {
			@apply mr-3 inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white;
		}
	}
</style>
