import { writable } from 'svelte/store';
import { state } from '$lib/state';
import { Subscription } from 'sse-z';
import { addLog, LogSchema } from '$lib/logStore';

interface SSEState {
	failed: boolean;
	opened: boolean;
	error: string | null;
}

export function subscribe() {
	subscription = new Subscription({
		url: location.origin + '/api/sse',
		eventSourceOptions: { withCredentials: true },
		onComplete() {
			status.update((status) => {
				return {
					...status,
					opened: false
				};
			});
		},
		onError(error) {
			status.set({ failed: true, opened: false, error: error.message });
		},
		onNext(data) {
			let parsed = LogSchema.safeParse(data);
			if (parsed.success) {
				addLog(parsed.data);
			} else {
				console.error(`Error parsing message: ${data}`);
			}
		}
	});
	subscription.eventSource.addEventListener('open', () => {
		status.set({ failed: false, opened: true, error: null });
	});
}

export function unsubscribe() {
	subscription.unsubscribe();
}

let subscription: Subscription;

const status = writable<SSEState>({ failed: false, opened: false, error: null });

status.subscribe((status) => {
	state.update((state) => {
		return {
			...state,
			connected: status.opened
		};
	});
});

export { status };
