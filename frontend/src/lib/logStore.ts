import { writable } from 'svelte/store';
import { z } from 'zod';

export const LogSchema = z.object({
	id: z.number(),
	date: z.number(),
	server_user: z.string(),
	client_ip: z.string(),
	client_name: z.string(),
	share_name: z.string(),
	action: z.string(),
	data: z.string().nullable(),
	path: z.string(),
	path2: z.string().nullable()
});

export type Log = z.infer<typeof LogSchema>;

export function loadLogs(log: Log[]) {
	logs.set(log);
}

export function addLog(log: Log) {
	logs.update((logs) => {
		logs.unshift(log);
		return logs;
	});
}

export const logs = writable<Log[]>([]);

export const filterSchema = z.object({
	oldest: z.number(),
	share_name: z.array(z.string()),
	client_name: z.array(z.string()),
	client_ip: z.array(z.string()),
	actions: z.array(z.string())
});

export type Filter = z.infer<typeof filterSchema>;

export const filters = writable<Filter>({
	oldest: 0,
	share_name: [],
	client_name: [],
	client_ip: [],
	actions: []
});

logs.subscribe(console.log);
