<script lang="ts">
	import { state, checkLogin } from '$lib/state';
	import { goto } from '$app/navigation';
	import { login } from '$lib/api';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import { scale } from 'svelte/transition';

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
		blocked = false;

	async function submit() {
		let res = await login(username, password);

		if (res.ok) {
			goto('/');
		} else {
			blocked = false;
			console.log(res);
		}
	}
</script>

<svelte:head>
	<title>Login - Samba Dashboard</title>
	<meta name="description" content="Samba Dashboard" />
</svelte:head>

<div class="login">
	<form on:submit|preventDefault={submit} class="flex flex-col w-96 rounded p-4" transition:scale>
		<p class="text-lg font-bold mb-4">Samba manager</p>

		<label class="block">
			<span class="block text-sm font-medium">Username</span>
			<input use:focus bind:value={username} type="text" disabled={blocked} />
		</label>
		<label class="block">
			<span class="block text-sm font-medium">Password</span>
			<input bind:value={password} type="password" disabled={blocked} />
		</label>
		<button type="submit" disabled={blocked}>Login</button>
	</form>
</div>

<style lang="sass">
	.login
		@apply absolute h-screen w-screen flex justify-center items-center
		form
			@apply flex flex-col w-96 rounded p-4
			background: var(--secondary)

	input[type='text'],
	input[type='password']
		border-color: var(--secondary-2)
		@apply bg-transparent mt-1 w-full px-3 py-1.5 border-b text-sm text-center focus:outline-none focus:border-sky-500 focus:mb-[1px] focus:border-b-0 focus:ring-1 focus:ring-sky-500 disabled:bg-slate-50 disabled:text-slate-500 disabled:border-slate-200 invalid:border-pink-500 invalid:text-pink-600 focus:invalid:border-pink-500 focus:invalid:ring-pink-500

	label
		@apply mb-4

	button
		@apply bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 focus:ring-1
</style>
