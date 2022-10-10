import { loadLogs, addLog, LogSchema, type Log, filterSchema, filters } from '$lib/logStore';
import { checkLogin, criticalError } from '$lib/state';

import { z } from 'zod';

function api(path: string, options: RequestInit | undefined = undefined) {
	let req = fetch(path, options).then(async (res) => {
		if (res.ok) {
			return res;
		}

		if (res.status === 401 || res.status === 403) {
			if (!(await checkLogin())) {
				throw new Error('Not logged in API error\n' + (await res.text()));
			}
		}

		console.error(await res.text());
		criticalError(`${res.status}\n${res.statusText}`);
		throw req;
		// throw new Error(`${res.status}\n${await res.text()}`);
	});

	return req;
}

export async function login(username: string, password: string) {
	let req = await api('/api/login', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ username, password })
	});

	return req;
}

export function state() {
	return api('/api/status');
}

interface Filters {
	share_name: string | undefined;
	client_name: string | undefined;
	client_ip: string | undefined;
	actions: string | undefined;
	before: number | undefined;
	after: number | undefined;
	search: string | undefined;
}

export async function getLog(filters: Filters) {
	let params = new URLSearchParams();
	for (const [key, value] of Object.entries(filters)) {
		if (value !== -1 && value !== undefined && value !== '') {
			params.set(key, value);
		}
	}

	let req = await api('/api/log?' + params, { method: 'POST' });
	if (req.ok) {
		let result = z.array(LogSchema).safeParse(await req.json());

		if (result.success) {
			loadLogs(result.data);
			return result.data;
		} else {
			criticalError(result.error.message);
			console.error(result.error);
			throw result.error;
		}
	} else {
		throw req;
	}
}

export async function getLatestLogs() {
	let req = await api('/api/log', {
		method: 'POST'
	});

	if (req.ok) {
		let result = z.array(LogSchema).safeParse(await req.json());

		if (result.success) {
			loadLogs(result.data);
		} else {
			return result.error;
		}
	} else {
		throw req;
	}
}

export async function getFilters() {
	let req = await api('/api/filters', { method: 'POST' });
	if (req.ok) {
		let result = filterSchema.safeParse(await req.json());

		if (result.success) {
			filters.set({
				actions: [...new Set(result.data.actions)],
				client_ip: [...new Set(result.data.client_ip)],
				client_name: [...new Set(result.data.client_name)],
				oldest: result.data.oldest,
				share_name: [...new Set(result.data.share_name)]
			});
			return true;
		} else {
			throw result.error;
		}
	} else {
		throw req;
	}
}

export async function logout() {
	let req = await api('/api/logout', {
		method: 'POST'
	});
	if (req.ok) {
		await checkLogin();
		loadLogs([]);
	} else {
		await checkLogin();
	}
	return req.ok;
}
