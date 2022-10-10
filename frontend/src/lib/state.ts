import { state as StateApi } from '$lib/api';
import { writable } from 'svelte/store';
import { goto } from '$app/navigation';

export async function checkLogin() {
	if (error) throw '';

	let req = await (await StateApi()).json();

	let loggedIn: boolean = req.logged_in;
	state.update((state) => {
		return {
			...state,
			loggedIn: loggedIn,
			needsLogin: !loggedIn
		};
	});

	!loggedIn && goto('/login');
	return loggedIn;
}

let error = false;
export function criticalError(err: unknown) {
	error = true;
	let data =
		typeof err === 'string'
			? err
			: err instanceof Error
			? err.stack ?? err.message
			: 'Unknown error';
	state.update((state) => {
		return {
			...state,
			criticalError: data
		};
	});
}

interface state {
	loggedIn: boolean;
	connected: boolean;
	fetched: boolean;
	needsLogin: boolean;
	gotFilters: boolean;
	criticalError: String | null;
}

export const state = writable<state>({
	loggedIn: false,
	connected: false,
	fetched: false,
	needsLogin: false,
	gotFilters: false,
	criticalError: null
});

state.subscribe(console.log);
