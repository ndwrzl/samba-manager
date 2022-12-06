<script lang="ts">
	import { state, checkLogin } from '$lib/state';
	import { goto } from '$app/navigation';
	import { login, NotLoggedIn } from '$lib/api';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import { scale } from 'svelte/transition';
	import { locales, locale, t } from '$lib/translations';

	onMount(() => {
		if (!$state.needsLogin) {
			if (browser && !$state.needsLogin) {
				goto('/');
			}
		}
		return checkLogin;
	});

	function focus(input: HTMLInputElement) {
		input.focus();
	}

	let password: string = '',
		username: string = '',
		blocked = false,
		error: any;

	async function submit() {
		blocked = true;
		error = null;
		let res;
		try {
			res = await login(username, password);
		} catch (e) {
			error = e;
			blocked = false;
			setTimeout(() => {
				error = null;
			}, 2000);
			return;
		}

		if (res.ok) {
			goto('/');
		}
		blocked = false;
	}
</script>

<svelte:head>
	<title>{$t('login.title')} - Samba Dashboard</title>
	<meta name="description" content="Samba Dashboard" />
</svelte:head>

<div class="login">
	<div class="form w-96 rounded p-4" transition:scale>
		<select
			bind:value={$locale}
			class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 mb-2"
		>
			{#each $locales as value}
				<option {value}>{$t(`lang.${value}`)}</option>
			{/each}
		</select>
		<form on:submit|preventDefault={submit} class="flex flex-col ">
			<p class="text-lg font-bold mb-4">Samba manager</p>

			{#if error}
				<div
					transition:scale
					class="bg-red-100 border border-red-400 text-red-700 text-sm px-2 py-3 mb-3 rounded relative"
					role="alert"
				>
					<strong class="font-bold">
						{#if error instanceof NotLoggedIn}
							{$t('login.invalidCredentials')}
						{:else}
							{$t('login.unknownError')}
						{/if}
					</strong>
				</div>
			{/if}

			<label class="block">
				<span class="block text-sm font-medium">{$t('login.username')}</span>
				<input use:focus bind:value={username} type="text" disabled={blocked} />
			</label>
			<label class="block">
				<span class="block text-sm font-medium">{$t('login.password')}</span>
				<input bind:value={password} type="password" disabled={blocked} />
			</label>
			<button type="submit" disabled={blocked}>{$t('login.login')}</button>
		</form>
	</div>
</div>

<style lang="scss">
	.login {
		@apply absolute h-screen w-screen flex justify-center items-center;
		.form {
			background: var(--secondary);
			@apply flex flex-col w-96 rounded p-4;
		}
	}

	input[type='text'],
	input[type='password'] {
		border-color: var(--secondary-2);
		@apply bg-transparent mt-1 w-full px-3 py-1.5 border-b text-sm text-center focus:outline-none focus:border-sky-500 focus:mb-[1px] focus:border-b-0 focus:ring-1 focus:ring-sky-500 disabled:border-slate-200 invalid:border-pink-500 invalid:text-pink-600 focus:invalid:border-pink-500 focus:invalid:ring-pink-500;
	}

	label {
		@apply mb-4;
	}

	button {
		@apply bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 focus:ring-1;
	}
</style>
