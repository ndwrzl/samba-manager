import { writable } from 'svelte/store';

interface Filters {
	share_name: number;
	client_name: number;
	client_ip: number;
	actions: number;
	before: number;
	after: number;
	search: string;
}

export const filters = writable<Filters>({
	share_name: -1,
	client_name: -1,
	client_ip: -1,
	actions: -1,
	before: -1,
	after: -1,
	search: ''
});
